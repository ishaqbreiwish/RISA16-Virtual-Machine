use risa16::assembler::assemble;
use risa16::decoder::decode;
use risa16::instructions::Instruction;
use risa16::vm::{State, VM};

//
// ---------- helpers ----------
//

fn run_program(src: &str) -> VM {
    let bytes = assemble(src).expect("assembly failed");

    let mut vm = VM::new();
    vm.memory.data[..bytes.len()].copy_from_slice(&bytes);

    while vm.state == State::RUNNING {
        vm.step();
    }

    vm
}

//
// ---------- assembler → bytes ----------
//

#[test]
fn assemble_movimm() {
    let src = "movimm r3 0x1234";
    let bytes = assemble(src).unwrap();

    assert_eq!(bytes, vec![0x01, 0x03, 0x12, 0x34]);
}

#[test]
fn assemble_add() {
    let src = "add r1 r2";
    let bytes = assemble(src).unwrap();

    assert_eq!(bytes, vec![0x05, 0x01, 0x02]);
}

#[test]
fn assemble_halt() {
    let src = "halt";
    let bytes = assemble(src).unwrap();

    assert_eq!(bytes, vec![0xFF]);
}

//
// ---------- labels ----------
//

#[test]
fn assemble_jump_label() {
    let src = r#"
        start:
            movimm r0 1
            jmp start
    "#;

    let bytes = assemble(src).unwrap();

    assert_eq!(
        bytes,
        vec![
            0x01, 0x00, 0x00, 0x01, // movimm r0,1
            0x08, 0x00, 0x00 // jmp start (pc = 0)
        ]
    );
}

#[test]
fn assemble_forward_label() {
    let src = r#"
        jmp end
        movimm r0 1
        end:
            halt
    "#;

    let bytes = assemble(src).unwrap();

    assert_eq!(
        bytes,
        vec![
            0x08, 0x00, 0x07, // jmp end (pc = 7)
            0x01, 0x00, 0x00, 0x01, 0xFF
        ]
    );
}

//
// ---------- assembler → decoder ----------
//

#[test]
fn assemble_decode_roundtrip() {
    let src = "movimm r2 0xBEEF";
    let bytes = assemble(src).unwrap();

    let decoded = decode(&bytes, 0).unwrap();

    match decoded.instr {
        Instruction::MovImm { reg, imm } => {
            assert_eq!(reg, 2);
            assert_eq!(imm, 0xBEEF);
        }
        _ => panic!("wrong instruction"),
    }

    assert_eq!(decoded.length, 4);
}

//
// ---------- assembler → vm execution ----------
//

#[test]
fn program_adds_numbers() {
    let src = r#"
        movimm r0 5
        movimm r1 7
        add r0 r1
        halt
    "#;

    let vm = run_program(src);

    assert_eq!(vm.cpu.registers[0], 12);
    assert!(!vm.carry_flag);
    assert!(!vm.zero_flag);
}

#[test]
fn program_uses_jumpz() {
    let src = r#"
        movimm r0 5
        movimm r1 5
        cmp r0 r1
        jmpz equal
        movimm r2 1
        halt
    equal:
        movimm r2 42
        halt
    "#;

    let vm = run_program(src);

    assert_eq!(vm.cpu.registers[2], 42);
}

#[test]
fn program_loop() {
    let src = r#"
        movimm r0 3      // counter
        movimm r1 1      // decrement
        movimm r2 0      // zero constant
    loop:
        sub r0 r1        // r0 -= 1
        cmp r0 r2        // compare r0 with 0
        jmpnz loop       // loop while r0 != 0
        halt
    "#;

    let vm = run_program(src);

    assert_eq!(vm.cpu.registers[0], 0);
}

//
// ---------- memory ----------
//

#[test]
fn store_then_load_big_endian() {
    let src = r#"
        movimm r1 0xABCD
        store 0x0100 r1
        load r2 0x0100
        halt
    "#;

    let vm = run_program(src);

    assert_eq!(vm.cpu.registers[2], 0xABCD);
    assert_eq!(vm.memory.data[0x0100], 0xAB);
    assert_eq!(vm.memory.data[0x0101], 0xCD);
}

//
// ---------- error cases ----------
//

#[test]
fn assemble_unknown_instruction_fails() {
    let src = "foo r0 r1";
    assert!(assemble(src).is_err());
}

#[test]
fn assemble_invalid_register_fails() {
    let src = "movimm r99 1";
    assert!(assemble(src).is_err());
}

#[test]
fn assemble_missing_operand_fails() {
    let src = "add r0";
    assert!(assemble(src).is_err());
}

#[test]
fn assemble_undefined_label_fails() {
    let src = "jmp missing";
    assert!(assemble(src).is_err());
}
