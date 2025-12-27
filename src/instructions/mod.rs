#[derive(Debug)]
pub enum Instruction {
    MovImm { reg: u8, imm: u16 },      // 0x01
    Mov { src_reg: u8, dest_reg: u8 }, // 0x02
    LoadReg { reg: u8, addr: u16 },    // 0x03
    Store { addr: u16, reg: u8 },      // 0x04
    Add { dest_reg: u8, src_reg: u8 }, // 0x05
    Sub { dest_reg: u8, src_reg: u8 }, // 0x06
    Compare { reg_1: u8, reg_2: u8 },  // 0x07
    Jump { addr: u16 },                // 0x08
    JumpZ { addr: u16 },               // 0x09
    JumpNZ { addr: u16 },              // 0x0A
    Halt,                              // 0xFF
}
