use risa16::assembler::assemble;
use risa16::vm::{State, VM};

fn main() {
    let src = r#"
        movimm r0 10000
        movimm r1 1
        movimm r2 0
        movimm r3 0x0100

    loop:
        sub r0 r1
        cmp r0 r2
        store 0x0100 r0
        load r4 0x0100
        jmpnz loop
        halt
    "#;

    let bytes = assemble(src).expect("assembly failed");

    let mut vm = VM::new();
    vm.memory.data[..bytes.len()].copy_from_slice(&bytes);

    while vm.state == State::RUNNING {
        vm.step();
    }

    println!("==== RISA16 Benchmark Results ====");
    println!("Final r0 value: {}", vm.cpu.registers[0]);
    println!("Total instructions executed: {}", vm.instruction_count);

    println!("\nInstruction mix:");
    for (opcode, count) in vm.opcode_counts.iter().enumerate() {
        if *count > 0 {
            println!("  opcode 0x{:02X}: {}", opcode, count);
        }
    }

    println!("\nFlags:");
    println!("  Zero flag: {}", vm.zero_flag);
    println!("  Carry flag: {}", vm.carry_flag);

    println!("\nMemory check:");
    println!("  MEM[0x0100] = {:02X}", vm.memory.data[0x0100]);
    println!("  MEM[0x0101] = {:02X}", vm.memory.data[0x0101]);
}
