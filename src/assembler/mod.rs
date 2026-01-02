use crate::instructions;
use crate::instructions::Instruction;
use std::collections::HashMap;
use std::env;
use std::fmt::format;
use std::fs;
use std::path::Path;

pub fn assemble(src: &str) -> Result<Vec<u8>, String> {
    // --snip--
    let mut bytecode: Vec<u8> = Vec::new();
    let contents = src;
    let tokens: Vec<Vec<String>> = tokenize(contents.to_string());

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
        pc += find_instr_length(instr.to_string())?;
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
                    return Err("mov expects 2 operands".into());
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
                    return Err("load expects 2 operands".into());
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
                    return Err("store expects 2 operands".into());
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
                    return Err("add expects 2 operands".into());
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
                    return Err("sub expects 2 operands".into());
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
                    return Err("cmp expects 2 operands".into());
                }

                // push dest register
                let reg_a = parse_register(&line_tokens[idx + 1])?;
                bytecode.push(reg_a);

                // push source register
                let reg_b = parse_register(&line_tokens[idx + 2])?;
                bytecode.push(reg_b);
            }

            "jmp" | "jmpz" | "jmpnz" => {
                let opcode = match instr.as_str() {
                    "jmp" => 0x08,
                    "jmpz" => 0x09,
                    "jmpnz" => 0x0A,
                    _ => unreachable!(),
                };

                bytecode.push(opcode);

                if line_tokens.len() - idx != 2 {
                    return Err("jump operations expects 1 operand".into());
                }

                let target = &line_tokens[idx + 1];
                let addr: u16 = if let Ok(num) = parse_u16(target) {
                    num
                } else {
                    *label_table
                        .get(target)
                        .ok_or("jmp operand references label that does not exist")?
                };

                let addr_hi: u8 = (addr >> 8) as u8;
                let addr_lo: u8 = addr as u8;

                bytecode.push(addr_hi);
                bytecode.push(addr_lo);
            }

            "halt" => {
                bytecode.push(0xFF);
            }

            _ => return Err(format!("ERROR: Unknown Operand: {}", instr)),
        }
    }

    Ok(bytecode)
}

fn find_instr_length(mut instruction: String) -> Result<u16, String> {
    instruction = instruction.to_lowercase();
    match instruction.as_str() {
        "movimm" => Ok(4),
        "mov" => Ok(3),
        "load" => Ok(4),
        "store" => Ok(4),
        "add" => Ok(3),
        "sub" => Ok(3),
        "cmp" => Ok(3),
        "jmp" => Ok(3),
        "jmpz" => Ok(3),
        "jmpnz" => Ok(3),
        "halt" => Ok(1),
        _ => return Err(format!("ERROR: Unknown Operand: {}", instruction)),
    }
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
