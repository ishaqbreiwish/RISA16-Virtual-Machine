use risa16::vm::{State, VM};

/// Helper: load raw bytecode into memory starting at address 0
fn load_program(vm: &mut VM, program: &[u8]) {
    vm.memory.data[..program.len()].copy_from_slice(program);
}

#[test]
fn vm_initializes_correctly() {
    let vm = VM::new();

    // PC starts at 0
    assert_eq!(vm.cpu.pc, 0);

    // All registers zeroed
    for reg in vm.cpu.registers.iter() {
        assert_eq!(*reg, 0);
    }

    // Flags cleared
    assert_eq!(vm.zero_flag, false);
    assert_eq!(vm.carry_flag, false);

    // VM starts running
    matches!(vm.state, State::RUNNING);

    // Memory size correct
    assert_eq!(vm.memory.data.len(), 4096);
}

#[test]
fn halt_instruction_stops_vm() {
    let mut vm = VM::new();

    // Bytecode: single HALT instruction
    // Replace 0x00 with your actual HALT opcode if different
    let program = [0x00];
    load_program(&mut vm, &program);

    // Run exactly one instruction
    vm.step();

    // VM should now be halted
    matches!(vm.state, State::HALTED);

    // PC should not advance past HALT
    assert_eq!(vm.cpu.pc, 0);
}

#[test]
fn pc_advances_on_non_halt_instruction() {
    let mut vm = VM::new();

    // Dummy non-HALT instruction (NOP or placeholder opcode)
    // Replace 0xFF with any opcode that is NOT HALT
    let program = [0x0A];
    load_program(&mut vm, &program);

    vm.step();

    // VM should still be running
    matches!(vm.state, State::RUNNING);

    // PC should advance by instruction length (assumed 1 byte for now)
    assert_eq!(vm.cpu.pc, 3);
}
