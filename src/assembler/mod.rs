use crate::instructions;
use crate::instructions::Instruction;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

pub fn assemble() -> Result<Vec<u8>, String> {
    // --snip--
    let mut bytecode: Vec<u8> = Vec::new();
    let args: Vec<String> = env::args().collect();

    let file_path = Path::new(&args[1]);

    match file_path.extension().and_then(|e| e.to_str()) {
        Some("risa") | Some("txt") => {}
        _ => {
            eprintln!("Invalid file type");
            return Err(format!("Cannot find file: {}", file_path.to_string_lossy()));
        }
    }

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let tokens: Vec<Vec<String>> = tokenize(contents);

    let mut pc: u16 = 0;
    let mut label_table: HashMap<String, u16> = HashMap::new();

    // create label table
    for line_tokens in tokens.iter() {
        let mut idx = 0;

        if line_tokens[0].ends_with(":") {
            let label = line_tokens[0].trim_end_matches(':').to_string();
            label_table.insert(label, pc);
            idx = 1;
        }

        // label only line
        if idx >= line_tokens.len() {
            continue;
        }

        // instruction
        let instr = &line_tokens[idx];
        pc += find_instr_length(instr.to_string());
    }

    // emit bytecode
    for line_tokens in tokens.iter() {
        let mut idx = 0;

        if line_tokens[0].ends_with(":") {
            idx += 1;
        }

        if idx >= line_tokens.len() {
            continue;
        }

        let mut instr = (&line_tokens[idx]).to_string();
        instr = instr.to_lowercase();
        match instr.as_str() {
            "movimm" => {
                bytecode.push(0x01);

                if line_tokens.len() - idx != 3 {
                    return Err("movimm expects 2 operands".into());
                }

                // push register
                let reg = parse_register(&line_tokens[idx + 1])?;
                bytecode.push(reg);

                // push immediate

                let imm: u16 = parse_u16(&line_tokens[idx + 2])?;
                let imm_hi: u8 = (imm >> 8) as u8;
                let imm_lo: u8 = imm as u8;

                bytecode.push(imm_hi);
                bytecode.push(imm_lo);
            }

            "mov" => {
                bytecode.push(0x02);

                if line_tokens.len() - idx != 3 {
                    return Err("movimm expects 2 operands".into());
                }

                // push dest register
                let dest_reg = parse_register(&line_tokens[idx + 1])?;
                bytecode.push(dest_reg);

                // push src register
                let src_reg = parse_register(&line_tokens[idx + 2])?;
                bytecode.push(src_reg);
            }

            "load" => {
                bytecode.push(0x03);

                if line_tokens.len() - idx != 3 {
                    return Err("movimm expects 2 operands".into());
                }

                // push register
                let reg = parse_register(&line_tokens[idx + 1])?;
                bytecode.push(reg);

                // push address
                let addr: u16 = parse_u16(&line_tokens[idx + 2])?;
                let addr_hi: u8 = (addr >> 8) as u8;
                let addr_lo: u8 = addr as u8;

                bytecode.push(addr_hi);
                bytecode.push(addr_lo);
            }

            "store" => {
                bytecode.push(0x04);

                if line_tokens.len() - idx != 3 {
                    return Err("movimm expects 2 operands".into());
                }

                // push address
                let addr: u16 = parse_u16(&line_tokens[idx + 1])?;
                let addr_hi: u8 = (addr >> 8) as u8;
                let addr_lo: u8 = addr as u8;

                bytecode.push(addr_hi);
                bytecode.push(addr_lo);

                // push register
                let reg = parse_register(&line_tokens[idx + 2])?;
                bytecode.push(reg);
            }

            "add" => {
                bytecode.push(0x05);

                if line_tokens.len() - idx != 3 {
                    return Err("movimm expects 2 operands".into());
                }

                // push dest register
                let dest_reg = parse_register(&line_tokens[idx + 1])?;
                bytecode.push(dest_reg);

                // push source register
                let src_reg = parse_register(&line_tokens[idx + 2])?;
                bytecode.push(src_reg);
            }

            "sub" => {
                bytecode.push(0x06);

                if line_tokens.len() - idx != 3 {
                    return Err("movimm expects 2 operands".into());
                }

                // push dest register
                let dest_reg = parse_register(&line_tokens[idx + 1])?;
                bytecode.push(dest_reg);

                // push source register
                let src_reg = parse_register(&line_tokens[idx + 2])?;
                bytecode.push(src_reg);
            }

            "cmp" => {
                bytecode.push(0x07);

                if line_tokens.len() - idx != 3 {
                    return Err("movimm expects 2 operands".into());
                }

                // push dest register
                let reg_a = parse_register(&line_tokens[idx + 1])?;
                bytecode.push(reg_a);

                // push source register
                let reg_b = parse_register(&line_tokens[idx + 2])?;
                bytecode.push(reg_b);
            }

            _ => eprintln!("ERROR: Unknown Operand: {}", instr),
        }
    }

    Ok(bytecode)
}

fn find_instr_length(mut instruction: String) -> u16 {
    instruction = instruction.to_lowercase();
    match instruction.as_str() {
        "movimm" => return 4,
        "mov" => return 3,
        "load" => return 4,
        "store" => return 4,
        "add" => return 3,
        "sub" => return 3,
        "cmp" => return 3,
        "jmp" => return 3,
        "jmpz" => return 3,
        "jmpnz" => return 3,
        "halt" => return 1,
        _ => eprintln!("ERROR: Unknown Operand: {}", instruction),
    }
    return 0;
}

fn parse_register(token: &str) -> Result<u8, String> {
    if !token.starts_with('r') {
        return Err("Expected register (r0–r15)".into());
    }

    let num = token[1..]
        .parse::<u8>()
        .map_err(|_| "Invalid register number")?;

    if num >= 0x10 {
        return Err("Register out of bounds".into());
    }

    Ok(num)
}

fn parse_u16(token: &str) -> Result<u16, String> {
    if token.starts_with("0x") {
        u16::from_str_radix(&token[2..], 16).map_err(|_| "Invalid hex number".into())
    } else {
        token.parse::<u16>().map_err(|_| "Invalid number".into())
    }
}

fn tokenize(contents: String) -> Vec<Vec<String>> {
    let mut tokenized_lines: Vec<Vec<String>> = Vec::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut current_line: Vec<String> = Vec::new();
        for word in line.split_whitespace() {
            if word.starts_with("//") {
                break;
            }
            current_line.push(word.to_string());
        }

        if !current_line.is_empty() {
            tokenized_lines.push(current_line);
        }
    }

    return tokenized_lines;
}
