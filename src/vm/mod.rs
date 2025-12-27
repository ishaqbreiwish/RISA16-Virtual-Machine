use std::panic;

use crate::cpu::CPU;
use crate::decoder::{DecodedInstruction, decode};
use crate::instructions::Instruction;

use crate::memory::Memory;

#[derive(PartialEq)]
pub enum State {
    HALTED,
    RUNNING,
}

pub struct VM {
    pub cpu: CPU,
    pub memory: Memory,
    pub zero_flag: bool,
    pub carry_flag: bool,
    pub state: State,
}

impl VM {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            memory: Memory::new(),
            zero_flag: false,
            carry_flag: false,
            state: State::RUNNING,
        }
    }

    pub fn step(&mut self) {
        if self.state == State::HALTED {
            return;
        }

        let decoded: Result<DecodedInstruction, String> = decode(&self.memory.data, self.cpu.pc);
        match decoded {
            // because it might be an error
            Ok(instr) => match instr.instr {
                Instruction::Halt => {
                    self.state = State::HALTED;
                    return;
                }
                _ => {
                    self.execute(instr.instr);
                    self.cpu.pc += instr.length as u16;
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                self.state = State::HALTED
            }
        }
    }

    pub fn execute(&mut self, decoded: Instruction) {
        // need to be &mut self to borrow the vm instnance and mutate it
        match decoded {
            Instruction::MovImm { reg, imm } => {
                if imm == 0 {
                    self.zero_flag = true;
                } else {
                    self.zero_flag = false;
                }
                if reg >= 0x10 {
                    eprintln!("Error: Register out of bounds");
                    self.state = State::HALTED;
                    return;
                }
                self.cpu.registers[reg as usize] = imm;
            }

            Instruction::Mov { src_reg, dest_reg } => {
                if src_reg >= 0x10 || dest_reg >= 0x10 {
                    eprintln!("Error: Register out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                if self.cpu.registers[dest_reg as usize] == 0 {
                    // all array indices must be usize
                    self.zero_flag = true;
                } else {
                    self.zero_flag = false;
                }

                self.cpu.registers[dest_reg as usize] = self.cpu.registers[src_reg as usize];
            }

            Instruction::Add { dest_reg, src_reg } => {
                if src_reg >= 0x10 || dest_reg >= 0x10 {
                    eprintln!("Error: Register out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                // do the addition in u32 so
                let sum: u32 = (self.cpu.registers[dest_reg as usize] as u32)
                    + (self.cpu.registers[src_reg as usize] as u32);
                if sum >= 0x10000 {
                    self.carry_flag = true;
                } else {
                    self.carry_flag = false;
                }
                self.cpu.registers[dest_reg as usize] = sum as u16;

                if sum as u16 == 0 {
                    self.zero_flag = true;
                } else {
                    self.zero_flag = false;
                }
            }

            _ => print!("placeholder"),
        }
    }
}
