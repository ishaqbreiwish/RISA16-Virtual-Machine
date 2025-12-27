use crate::instructions::Instruction;

pub struct DecodedInstruction {
    pub instr: Instruction,
    pub length: u16,
}

pub fn decode(bytes: &[u8], pc: u16) -> Result<DecodedInstruction, String> {
    // 1. read opcode at pc
    // 2. read operands based on opcode
    // 3. construct Instruction
    // 4. return instruction + length

    let pc = pc as usize;
    let opcode = *bytes.get(pc).ok_or("PC out of bounds")?; // bytes.get() returns a pointer so we deref

    match opcode {
        0x01 => {
            // MoveImm: reg (1B), imm16 (2B)
            let reg = *bytes.get(pc + 1).ok_or("Missing Reg Byte")?;
            let imm_hi = *bytes.get(pc + 2).ok_or("Missing imm_hi Byte")?;
            let imm_lo = *bytes.get(pc + 3).ok_or("Missing imm_lo byte")?;

            let imm = ((imm_hi as u16) << 8) | (imm_lo as u16);

            Ok(DecodedInstruction {
                instr: Instruction::MovImm { reg, imm },
                length: 4,
            })
        }

        0x02 => {
            // Mov: dest_reg (1B), src_reg (1B)
            let dest_reg = *bytes.get(pc + 1).ok_or("Missing dest_reg Byte")?;
            let src_reg = *bytes.get(pc + 2).ok_or("Missing src_reg Byte")?;

            Ok(DecodedInstruction {
                instr: Instruction::Mov { dest_reg, src_reg },
                length: 3,
            })
        }

        0x03 => {
            // LoadReg: reg (1B), addr (2B) }
            let reg = *bytes.get(pc + 1).ok_or("Missing register Byte")?;
            let addr_hi = *bytes.get(pc + 2).ok_or("Missing register Byte")?;
            let addr_lo = *bytes.get(pc + 3).ok_or("Missing register Byte")?;

            let addr = ((addr_hi as u16) << 8) | (addr_lo as u16);

            Ok(DecodedInstruction {
                instr: Instruction::LoadReg { reg, addr },
                length: 4,
            })
        }

        0x04 => {
            // Store: addr: (2B), reg: (1B)
            let addr_hi = *bytes.get(pc + 1).ok_or("Missing register Byte")?;
            let addr_lo = *bytes.get(pc + 2).ok_or("Missing register Byte")?;
            let reg = *bytes.get(pc + 3).ok_or("Missing register Byte")?;

            let addr = ((addr_hi as u16) << 8) | (addr_lo as u16);

            Ok(DecodedInstruction {
                instr: Instruction::Store { addr, reg },
                length: 4,
            })
        }

        0x05 => {
            // Add: dest_reg (1B), src_reg (1B)
            let dest_reg = *bytes
                .get(pc + 1)
                .ok_or("Missing destination register Byte")?;
            let src_reg = *bytes.get(pc + 2).ok_or("Missing source register Byte")?;

            Ok(DecodedInstruction {
                instr: Instruction::Add { dest_reg, src_reg },
                length: 3,
            })
        }

        0x06 => {
            // Sub: dest_reg (1B), src_reg (1B)
            let dest_reg = *bytes
                .get(pc + 1)
                .ok_or("Missing destination register Byte")?;
            let src_reg = *bytes.get(pc + 2).ok_or("Missing source register Byte")?;

            Ok(DecodedInstruction {
                instr: Instruction::Sub { dest_reg, src_reg },
                length: 3,
            })
        }

        0x07 => {
            // Compare: reg_1 (1B) reg_2 (1B) }
            let reg_1 = *bytes.get(pc + 1).ok_or("Missing register_1 Byte")?;
            let reg_2 = *bytes.get(pc + 2).ok_or("Missing register_2 Byte")?;

            Ok(DecodedInstruction {
                instr: Instruction::Compare { reg_1, reg_2 },
                length: 3,
            })
        }

        0x08 => {
            // Jump addr (2B)
            let addr_hi = *bytes.get(pc + 1).ok_or("Missing address_hi byte")?;
            let addr_lo = *bytes.get(pc + 2).ok_or("Missing address_hi byte")?;

            let addr = ((addr_hi as u16) << 8) | (addr_lo as u16);
            Ok(DecodedInstruction {
                instr: Instruction::Jump { addr },
                length: 3,
            })
        }

        0x09 => {
            // JumpZ (2B)
            let addr_hi = *bytes.get(pc + 1).ok_or("Missing address_hi byte")?;
            let addr_lo = *bytes.get(pc + 2).ok_or("Missing address_hi byte")?;

            let addr = ((addr_hi as u16) << 8) | (addr_lo as u16);
            Ok(DecodedInstruction {
                instr: Instruction::JumpZ { addr },
                length: 3,
            })
        }

        0x0A => {
            // JumpNZ (2B)
            let addr_hi = *bytes.get(pc + 1).ok_or("Missing address_hi byte")?;
            let addr_lo = *bytes.get(pc + 2).ok_or("Missing address_hi byte")?;

            let addr = ((addr_hi as u16) << 8) | (addr_lo as u16);
            Ok(DecodedInstruction {
                instr: Instruction::JumpNZ { addr },
                length: 3,
            })
        }

        0xFF => {
            // HALT
            Ok(DecodedInstruction {
                instr: Instruction::Halt,
                length: (1),
            })
        }
        _ => Err("Error: Invalid Opcode".to_string()),
    }
}
