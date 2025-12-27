use risa16::instructions::Instruction;
use risa16::vm::{State, VM};

//
// -------- helpers --------
//

fn fresh_vm() -> VM {
    VM::new()
}

//
// -------- initialization --------
//

#[test]
fn vm_initial_state() {
    let vm = fresh_vm();

    assert_eq!(vm.state, State::RUNNING);
    assert_eq!(vm.cpu.pc, 0);
    assert!(!vm.zero_flag);
    assert!(!vm.carry_flag);

    for r in vm.cpu.registers.iter() {
        assert_eq!(*r, 0);
    }

    assert_eq!(vm.memory.data.len(), 4096);
}

//
// -------- MOV / MOV_IMM --------
//

#[test]
fn mov_imm_writes_register() {
    let mut vm = fresh_vm();

    vm.execute(Instruction::MovImm {
        reg: 3,
        imm: 0x1234,
    });
    assert_eq!(vm.cpu.registers[3], 0x1234);
}

#[test]
fn mov_copies_register() {
    let mut vm = fresh_vm();

    vm.cpu.registers[1] = 42;
    vm.execute(Instruction::Mov {
        src_reg: 1,
        dest_reg: 2,
    });

    assert_eq!(vm.cpu.registers[2], 42);
}

//
// -------- ADD --------
//

#[test]
fn add_basic() {
    let mut vm = fresh_vm();

    vm.cpu.registers[0] = 5;
    vm.cpu.registers[1] = 7;

    vm.execute(Instruction::Add {
        dest_reg: 0,
        src_reg: 1,
    });

    assert_eq!(vm.cpu.registers[0], 12);
    assert!(!vm.carry_flag);
    assert!(!vm.zero_flag);
}

#[test]
fn add_sets_carry_and_zero() {
    let mut vm = fresh_vm();

    vm.cpu.registers[0] = 0xFFFF;
    vm.cpu.registers[1] = 1;

    vm.execute(Instruction::Add {
        dest_reg: 0,
        src_reg: 1,
    });

    assert_eq!(vm.cpu.registers[0], 0);
    assert!(vm.carry_flag);
    assert!(vm.zero_flag);
}

//
// -------- SUB --------
//

#[test]
fn sub_basic() {
    let mut vm = fresh_vm();

    vm.cpu.registers[0] = 10;
    vm.cpu.registers[1] = 3;

    vm.execute(Instruction::Sub {
        dest_reg: 0,
        src_reg: 1,
    });

    assert_eq!(vm.cpu.registers[0], 7);
    assert!(!vm.carry_flag);
}

#[test]
fn sub_sets_borrow_and_zero() {
    let mut vm = fresh_vm();

    vm.cpu.registers[0] = 3;
    vm.cpu.registers[1] = 3;

    vm.execute(Instruction::Sub {
        dest_reg: 0,
        src_reg: 1,
    });

    assert_eq!(vm.cpu.registers[0], 0);
    assert!(!vm.carry_flag);
    assert!(vm.zero_flag);
}

//
// -------- CMP --------
//

#[test]
fn cmp_equal_sets_zero() {
    let mut vm = fresh_vm();

    vm.cpu.registers[0] = 5;
    vm.cpu.registers[1] = 5;

    vm.execute(Instruction::Compare { reg_1: 0, reg_2: 1 });

    assert!(vm.zero_flag);
    assert!(!vm.carry_flag);
}

#[test]
fn cmp_less_sets_carry() {
    let mut vm = fresh_vm();

    vm.cpu.registers[0] = 3;
    vm.cpu.registers[1] = 7;

    vm.execute(Instruction::Compare { reg_1: 0, reg_2: 1 });

    assert!(!vm.zero_flag);
    assert!(vm.carry_flag);
}

//
// -------- JUMPS --------
//

#[test]
fn jump_sets_pc() {
    let mut vm = fresh_vm();

    vm.execute(Instruction::Jump { addr: 0x200 });
    assert_eq!(vm.cpu.pc, 0x200);
}

#[test]
fn jumpz_taken() {
    let mut vm = fresh_vm();

    vm.zero_flag = true;
    vm.execute(Instruction::JumpZ { addr: 0x300 });

    assert_eq!(vm.cpu.pc, 0x300);
}

#[test]
fn jumpz_not_taken() {
    let mut vm = fresh_vm();

    vm.zero_flag = false;
    vm.execute(Instruction::JumpZ { addr: 0x300 });

    assert_eq!(vm.cpu.pc, 0);
}

#[test]
fn jumpnz_taken() {
    let mut vm = fresh_vm();

    vm.zero_flag = false;
    vm.execute(Instruction::JumpNZ { addr: 0x400 });

    assert_eq!(vm.cpu.pc, 0x400);
}

//
// -------- LOAD / STORE --------
//

#[test]
fn store_then_load_roundtrip_big_endian() {
    let mut vm = fresh_vm();

    vm.cpu.registers[1] = 0xABCD;
    vm.execute(Instruction::Store {
        addr: 0x100,
        reg: 1,
    });

    assert_eq!(vm.memory.data[0x100], 0xAB);
    assert_eq!(vm.memory.data[0x101], 0xCD);

    vm.cpu.registers[2] = 0;
    vm.execute(Instruction::Load {
        addr: 0x100,
        reg: 2,
    });

    assert_eq!(vm.cpu.registers[2], 0xABCD);
}

//
// -------- HALT --------
//

#[test]
fn halt_stops_vm() {
    let mut vm = fresh_vm();

    vm.execute(Instruction::Halt);
    assert_eq!(vm.state, State::HALTED);
}

//
// -------- ERROR CASES --------
//

#[test]
fn invalid_register_halts_vm() {
    let mut vm = fresh_vm();

    vm.execute(Instruction::MovImm { reg: 99, imm: 1 });
    assert_eq!(vm.state, State::HALTED);
}

#[test]
fn invalid_memory_address_halts_vm() {
    let mut vm = fresh_vm();

    vm.execute(Instruction::Load {
        reg: 0,
        addr: 0x0FFF,
    });
    assert_eq!(vm.state, State::HALTED);
}
