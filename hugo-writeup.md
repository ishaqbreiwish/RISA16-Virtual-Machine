---
title: "RISA-16"
date: 2026-02-06
draft: false
tags: ["rust", "systems-programming", "virtual-machine", "computer-architecture"]
description: "A 16-bit virtual machine with custom ISA and two-pass assembler, built from scratch in Rust."
---

# RISA-16

A complete 16-bit virtual machine built from first principles in Rust.

## What It Is

A fully functional VM with its own instruction set architecture, a two-pass assembler that compiles custom assembly to bytecode, and a deterministic execution engine.

## Key Features

- **Custom ISA** with 11 instructions, 16 registers, and 4 KB memory
- **Two-pass assembler** with forward/backward label resolution
- **Bounds-checked decoder** for safe bytecode interpretation
- **Fetch-decode-execute pipeline** with Zero and Carry flags
- **Big-endian architecture** with correct word-aligned memory access
- **45 tests** covering assembler, decoder, and VM execution

## Example Program

```asm
movimm r0 3      ; counter
movimm r1 1      ; decrement value

loop:
    sub r0 r1
    jmpnz loop
    halt
```

## Technical Stack

**Language:** Rust  
**Test Coverage:** 45 unit and integration tests  
**Architecture:** Modular (CPU, Memory, Decoder, Assembler, VM)

## What I Learned

Building this required understanding ISA design tradeoffs, compiler internals like two-pass assembly and symbol tables, memory models and endianness, and execution semantics at the hardware level. I made sure every component is tested and edge cases are handled carefully. If an invalid register is used, the system halts with a clear error instead of failing silently or corrupting data.
