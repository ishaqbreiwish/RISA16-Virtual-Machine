pub mod assembler;
pub mod cpu;
pub mod decoder;
pub mod instructions;
pub mod memory;
pub mod vm;
pub mod graphics;
use wasm_bindgen::prelude::*;
use assembler::assemble;
use vm::{State, VM};

#[wasm_bindgen]
pub fn run(src: &str) -> String {
    let bytes = match assemble(src) {
        Err(e) => return format!(r#"{{"error":"{}"}}"#, e),
        Ok(b) => b,
    };

    let mut vm = VM::new();
    vm.memory.data[..bytes.len()].copy_from_slice(&bytes);

    let mut steps = 0u64;
    while vm.state == State::RUNNING && steps < 100_000 {
        vm.step();
        steps += 1;
    }

    let regs: Vec<String> = vm.cpu.registers
        .iter()
        .enumerate()
        .map(|(i, v)| format!(r#""r{}":{}"#, i, v))
        .collect();

    let fb: Vec<String> = vm.framebuffer.pixels.iter().map(|p| p.to_string()).collect();

    format!(
        r#"{{"registers":{{{}}}, "zero_flag":{}, "carry_flag":{}, "instruction_count":{}, "halted":{}, "framebuffer":[{}]}}"#,
        regs.join(","),
        vm.zero_flag,
        vm.carry_flag,
        vm.instruction_count,
        vm.state == State::HALTED,
        fb.join(","),
    )
}
