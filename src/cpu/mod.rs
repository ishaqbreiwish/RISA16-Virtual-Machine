// src/cpu/mod.rs
pub struct CPU {
    pub registers: [u16; 16], // 16 general-purpose 16-bit registers
    pub pc: u16,              // program counter
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            pc: 0,
        }
    }
}
