use risa16::decoder::decode;
use risa16::instructions::Instruction;

#[test]
fn decode_mov_imm() {
    // MOV_IMM r3, 0x1234
    let bytes = vec![0x01, 0x03, 0x12, 0x34];
    let pc = 0;

    let decoded = decode(&bytes, pc).expect("decode failed");

    match decoded.instr {
        Instruction::MovImm { reg, imm } => {
            assert_eq!(reg, 3);
            assert_eq!(imm, 0x1234);
        }
        _ => panic!("Expected MovImm instruction"),
    }

    assert_eq!(decoded.length, 4);
}

#[test]
fn decode_mov_reg() {
    // MOV r1, r7
    let bytes = vec![0x02, 0x01, 0x07];
    let pc = 0;

    let decoded = decode(&bytes, pc).expect("decode failed");

    match decoded.instr {
        Instruction::Mov { dest_reg, src_reg } => {
            assert_eq!(dest_reg, 1);
            assert_eq!(src_reg, 7);
        }
        _ => panic!("Expected Mov instruction"),
    }

    assert_eq!(decoded.length, 3);
}

#[test]
fn decode_invalid_opcode() {
    let bytes = vec![0xAA];
    let pc = 0;

    let result = decode(&bytes, pc);

    assert!(result.is_err());
}

#[test]
fn decode_out_of_bounds_pc() {
    let bytes = vec![0x01];
    let pc = 10;

    let result = decode(&bytes, pc);

    assert!(result.is_err());
}

#[test]
fn decode_incomplete_mov_imm() {
    // Missing imm bytes
    let bytes = vec![0x01, 0x02];
    let pc = 0;

    let result = decode(&bytes, pc);

    assert!(result.is_err());
}

#[test]
fn decode_load_reg() {
    // LOAD r2, [0x1234]
    let bytes = vec![0x03, 0x02, 0x12, 0x34];

    let decoded = decode(&bytes, 0).expect("decode failed");

    match decoded.instr {
        Instruction::Load { reg, addr } => {
            assert_eq!(reg, 2);
            assert_eq!(addr, 0x1234);
        }
        _ => panic!("Expected Load"),
    }

    assert_eq!(decoded.length, 4);
}

#[test]
fn decode_store() {
    // STORE [0x2000], r5
    let bytes = vec![0x04, 0x20, 0x00, 0x05];

    let decoded = decode(&bytes, 0).expect("decode failed");

    match decoded.instr {
        Instruction::Store { addr, reg } => {
            assert_eq!(addr, 0x2000);
            assert_eq!(reg, 5);
        }
        _ => panic!("Expected Store"),
    }

    assert_eq!(decoded.length, 4);
}

#[test]
fn decode_add() {
    // ADD r1, r2
    let bytes = vec![0x05, 0x01, 0x02];

    let decoded = decode(&bytes, 0).expect("decode failed");

    match decoded.instr {
        Instruction::Add { dest_reg, src_reg } => {
            assert_eq!(dest_reg, 1);
            assert_eq!(src_reg, 2);
        }
        _ => panic!("Expected Add"),
    }

    assert_eq!(decoded.length, 3);
}

#[test]
fn decode_sub() {
    // SUB r4, r3
    let bytes = vec![0x06, 0x04, 0x03];

    let decoded = decode(&bytes, 0).expect("decode failed");

    match decoded.instr {
        Instruction::Sub { dest_reg, src_reg } => {
            assert_eq!(dest_reg, 4);
            assert_eq!(src_reg, 3);
        }
        _ => panic!("Expected Sub"),
    }

    assert_eq!(decoded.length, 3);
}

#[test]
fn decode_compare() {
    // CMP r6, r7
    let bytes = vec![0x07, 0x06, 0x07];

    let decoded = decode(&bytes, 0).expect("decode failed");

    match decoded.instr {
        Instruction::Compare { reg_1, reg_2 } => {
            assert_eq!(reg_1, 6);
            assert_eq!(reg_2, 7);
        }
        _ => panic!("Expected Compare"),
    }

    assert_eq!(decoded.length, 3);
}

#[test]
fn decode_jump() {
    // JMP 0x0100
    let bytes = vec![0x08, 0x01, 0x00];

    let decoded = decode(&bytes, 0).expect("decode failed");

    match decoded.instr {
        Instruction::Jump { addr } => {
            assert_eq!(addr, 0x0100);
        }
        _ => panic!("Expected Jump"),
    }

    assert_eq!(decoded.length, 3);
}

#[test]
fn decode_jump_z() {
    // JZ 0x0020
    let bytes = vec![0x09, 0x00, 0x20];

    let decoded = decode(&bytes, 0).expect("decode failed");

    match decoded.instr {
        Instruction::JumpZ { addr } => {
            assert_eq!(addr, 0x0020);
        }
        _ => panic!("Expected JumpZ"),
    }

    assert_eq!(decoded.length, 3);
}

#[test]
fn decode_jump_nz() {
    // JNZ 0x0040
    let bytes = vec![0x0A, 0x00, 0x40];

    let decoded = decode(&bytes, 0).expect("decode failed");

    match decoded.instr {
        Instruction::JumpNZ { addr } => {
            assert_eq!(addr, 0x0040);
        }
        _ => panic!("Expected JumpNZ"),
    }

    assert_eq!(decoded.length, 3);
}

#[test]
fn decode_halt() {
    let bytes = vec![0xFF];

    let decoded = decode(&bytes, 0).expect("decode failed");

    match decoded.instr {
        Instruction::Halt => {}
        _ => panic!("Expected Halt"),
    }

    assert_eq!(decoded.length, 1);
}
