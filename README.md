# RISA16 Virtual Machine

RISA16 is a custom 16-bit virtual machine implemented in Rust.
The project includes a complete toolchain consisting of a formally defined instruction set architecture (ISA), a two-pass assembler, a bytecode decoder, and a deterministic execution engine.

The project focuses on correctness, safety, and testability, and was built from first principles to model a realistic virtual machine architecture at a small scale.
---

## Project Overview

RISA16 models a simple 16-bit CPU with a fixed register file, flags, and flat memory. Programs are written in a custom assembly language, assembled into bytecode, decoded at runtime, and executed instruction by instruction by the virtual machine.

The project is structured to mirror real systems design:

* a formal ISA specification
* a pure assembler implementation
* a bounds-checked decoder
* a stateful execution engine
* comprehensive unit and integration tests

---

## Architecture Summary

* **Register width:** 16-bit
* **General-purpose registers:** 16 (R0–R15)
* **Program counter:** 16-bit, byte-addressed
* **Memory:** 4 KB flat byte-addressable memory
* **Endianness:** Big-endian
* **Flags:** Zero (Z), Carry (C)
* **Execution model:** Fetch → Decode → Execute
* **Execution granularity:** One instruction per step

All memory accesses, register accesses, and jumps are bounds-checked. Invalid operations halt execution.

---

## Instruction Set

RISA16 defines a compact instruction set covering:

* data movement
* arithmetic
* comparison
* control flow
* memory access
* program termination

Instructions are encoded as bytecode and decoded dynamically at runtime.

Examples include:

* `movimm` — load immediate into register
* `mov` — register to register move
* `add`, `sub` — arithmetic operations
* `cmp` — register comparison (sets flags)
* `jmp`, `jmpz`, `jmpnz` — control flow
* `load`, `store` — memory access
* `halt` — stop execution

The complete instruction set, binary formats, and execution semantics are defined in **SPEC.md**.

---

## Assembler

The assembler translates RISA16 assembly into bytecode.

Key properties:

* Two-pass design

  * Pass 1 builds a label table and computes instruction addresses
  * Pass 2 emits bytecode
* Supports forward and backward label references
* Emits big-endian encoded instructions
* Clean error handling for:

  * invalid instructions
  * invalid registers
  * invalid operands
  * undefined labels

The assembler is implemented as a pure function and is fully testable.

---

## Virtual Machine

The virtual machine executes bytecode deterministically.

Execution flow:

1. Fetch opcode at PC
2. Decode instruction and operands
3. Execute instruction
4. Advance PC unless modified by control flow

Execution halts explicitly via the `halt` instruction or on invalid operations.

---

## Example Program

```asm
movimm r0 3
movimm r1 1
movimm r2 0

loop:
    sub r0 r1
    cmp r0 r2
    jmpnz loop
    halt
```

This program decrements a register until it reaches zero, then halts.

---

## Testing

The project includes comprehensive unit and integration tests covering:

* assembler correctness
* decoder correctness
* individual instruction behavior
* memory semantics
* flag behavior
* label resolution
* full assembly → execution pipelines
* error conditions

Run all tests with:

```bash
cargo test
```

---

## Design Goals

* Correctness over performance
* Clear separation of concerns
* Safe execution with explicit bounds checking
* Deterministic behavior
* Test-driven validation
* Readable, maintainable code

---

## Status

The RISA16 virtual machine, assembler, and decoder are complete and fully tested.
In the future, I want to extend it to support graphics.

---
