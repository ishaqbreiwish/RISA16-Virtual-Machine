use crate::cpu::CPU;
use crate::decoder::{DecodedInstruction, decode};
use crate::instructions::Instruction;

use crate::memory::Memory;

#[derive(PartialEq, Debug)]
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
            Ok(instr) => {
                let old_pc: u16 = self.cpu.pc;
                self.execute(instr.instr);
                if self.state == State::HALTED {
                    return;
                }
                if old_pc == self.cpu.pc {
                    // if pc mutated in execute ie. if a jump occured we dont want to change it
                    self.cpu.pc += instr.length as u16;
                }
            }
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
                // out of bounds check
                if reg >= 0x10 {
                    eprintln!("Error: Register out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                // R[reg] = imm
                self.cpu.registers[reg as usize] = imm;
            }

            Instruction::Mov { src_reg, dest_reg } => {
                // out of bounds check
                if src_reg >= 0x10 || dest_reg >= 0x10 {
                    eprintln!("Error: Register out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                // set zero flag
                self.zero_flag = self.cpu.registers[dest_reg as usize] == 0;

                // R[dest_reg] = R[src_reg]
                self.cpu.registers[dest_reg as usize] = self.cpu.registers[src_reg as usize];
            }

            Instruction::Add { dest_reg, src_reg } => {
                // out of bounds check
                if src_reg >= 0x10 || dest_reg >= 0x10 {
                    eprintln!("Error: Register out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                // do the addition in u32 so
                let sum: u32 = (self.cpu.registers[dest_reg as usize] as u32)
                    + (self.cpu.registers[src_reg as usize] as u32);

                // set carry flag
                self.carry_flag = sum >= 0x10000;

                // R[dest_reg] = sum
                self.cpu.registers[dest_reg as usize] = sum as u16;

                // set zero flag
                self.zero_flag = sum as u16 == 0;
            }

            Instruction::Sub { dest_reg, src_reg } => {
                // out of bounds check
                if src_reg >= 0x10 || dest_reg >= 0x10 {
                    eprintln!("Error: Register out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                // set carry flag
                self.carry_flag =
                    self.cpu.registers[dest_reg as usize] < self.cpu.registers[src_reg as usize];

                // do the addition in u32 so
                let diff: u32 = (self.cpu.registers[dest_reg as usize] as u32)
                    - (self.cpu.registers[src_reg as usize] as u32);

                // R[dest_reg] = difference
                self.cpu.registers[dest_reg as usize] = diff as u16;

                // set zero flag
                self.zero_flag = diff as u16 == 0
            }

            Instruction::Compare { reg_1, reg_2 } => {
                // out of bounds check
                if reg_1 >= 0x10 || reg_2 >= 0x10 {
                    eprintln!("Error: Register out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                let reg_1_value: u16 = self.cpu.registers[reg_1 as usize];
                let reg_2_value: u16 = self.cpu.registers[reg_2 as usize];

                self.zero_flag = reg_1_value == reg_2_value;
                self.carry_flag = reg_1_value < reg_2_value;
            }

            Instruction::Jump { addr } => {
                // out of bounds check
                if addr >= 0x1000 {
                    eprintln!("Error: address out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                self.cpu.pc = addr;
            }

            Instruction::JumpZ { addr } => {
                // out of bounds check
                if addr >= 0x1000 {
                    eprintln!("Error: address out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                if self.zero_flag == true {
                    self.cpu.pc = addr;
                }
            }

            Instruction::JumpNZ { addr } => {
                // out of bounds check
                if addr >= 0x1000 {
                    eprintln!("Error: address out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                if self.zero_flag == false {
                    self.cpu.pc = addr;
                }
            }

            Instruction::Store { addr, reg } => {
                // out of bounds checks for memory and register
                if addr >= 0x1000 || addr + 1 >= 0x1000 {
                    eprintln!("Error: address out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                if reg >= 0x10 {
                    eprintln!("Error: Register out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                let reg_value: u16 = self.cpu.registers[reg as usize];
                self.memory.data[addr as usize] = (reg_value >> 8) as u8; // move 8 bits to the right then u8 takes lowest 8 bytes
                self.memory.data[(addr + 1) as usize] = reg_value as u8; // u8 will already take lower 8 bytes
            }

            Instruction::Load { reg, addr } => {
                // out of bounds checks for memory and register
                if addr >= 0x1000 || addr + 1 >= 0x1000 {
                    eprintln!("Error: address out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                if reg >= 0x10 {
                    eprintln!("Error: Register out of bounds");
                    self.state = State::HALTED;
                    return;
                }

                let memory_value: u16 = (self.memory.data[addr as usize] as u16) << 8
                    | (self.memory.data[(addr + 1) as usize]) as u16;
                self.cpu.registers[reg as usize] = memory_value;
            }

            Instruction::Halt => {
                self.state = State::HALTED;
            }
        }
    }
}
