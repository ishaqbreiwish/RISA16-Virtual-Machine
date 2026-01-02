use risa16::assembler::assemble;
use risa16::vm::{State, VM};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: risa16 <file>");
        std::process::exit(1);
    }

    let src = fs::read_to_string(&args[1]).expect("Failed to read source file");

    let bytecode = assemble(&src).expect("Assembly failed");

    let mut vm = VM::new();
    vm.memory.data[..bytecode.len()].copy_from_slice(&bytecode);

    while vm.state == State::RUNNING {
        vm.step();
    }

    println!("Final registers:");
    for (i, r) in vm.cpu.registers.iter().enumerate() {
        println!("R{} = {:#06X}", i, r);
    }
}
