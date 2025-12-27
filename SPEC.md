# RISA Specification

## 1. Architectural Overview
- Register size: 16-bit
- Address size: 16-bit (max memory: 65536 bytes)
- Max addressable memory: 65,536 bytes
- Endianness: Big-endian
- Execution model: Fetch → Decode → Execute
- Execution granularity: one instruction per step
- Target ISA size: ~20–25 instructions

---

## 2. Register File
- General-purpose registers: R0–R15 (16-bit)
- Program Counter (PC): 16-bit, byte-addressed
- Flags:
  - Zero (Z)
  - Carry (C)
- Optional: Stack Pointer (SP)
---

## 3. Memory Layout
- Memory is a flat, linear, byte-addressed array
- Total size: 65,536 bytes
- Addresses range from 0x0000 to 0xFFFF

### 3.1 - Word Access
- A word = 16 bits (2 bytes)
- Multi-byte values are stored in big-endian order
- High byte at `address`
- Low byte at `address + 1`

### 3.2 - Alignment
- LOAD and STORE access two consecutive bytes
- Misaligned accesses are allowed (no traps)
- Behavior is deterministic but not optimized

The VM owns all memory. There is no segmentation or protection.

## 4. Instruction Execution Rules
- PC always points to the next instruction opcode
- After execution:
  - PC advances by instruction length
  - *unless explicitly* modified by a control-flow instruction
  - Instructions execute sequentially

No speculative execution or pipelining

---
## 5. ISA Guide

### 5.1 - Move Immediate to Register

**Opcode:** `0x01`  
**Category:** Data Movement

**Description:**  
Loads a 16-bit immediate value into one of the 16 general-purpose registers (R0–R15).

**Operands:**  
- **Register (1 byte):** Index from 0–15 representing R0–R15.  
- **Immediate (2 bytes, big-endian):** A 16-bit constant value.

**Instruction Length:**  
4 bytes  
(1 opcode + 1 register + 2 immediate bytes)

**Binary Format:**  
[01] [reg] [imm_hi] [imm_lo]

**Execution Semantics:**  
Register `reg` is set to the 16-bit immediate value:  
```R[reg] ← imm```

**Flags Affected:**  
- **Zero (Z):** Set if the loaded immediate equals 0.  
- **Carry (C):** Unaffected.

**Example Encoding:**  
`01 03 12 34`  
Loads value `0x1234` into register R3.

### 5.2 - Move Register to Register

**Opcode:** `0x02`  
**Category:** Data Movement

**Description:**  
Loads a 16-bit value from a register (R0-R15) into another one of the 16 general-purpose registers (R0–R15).

**Operands:**  
- **Destination Register (1 byte):** Index from 0–15 representing R0–R15.  
- **Source Register (1 byte):** Index from 0–15 representing R0–R15.

**Instruction Length:**  
3 bytes  
(1 opcode + 1 register + 1 register)

**Binary Format:**  
[02] [dest_reg] [source_reg]

**Execution Semantics:**  
```R[dest_reg] ← R[src_reg]```

**Flags Affected:**  
- **Zero (Z):** Set if the copied value equals 0.  
- **Carry (C):** Unaffected.

**Example Encoding:**  
`02 04 07`  
Copies the value of R7 into R4.

### 5.3 - Load to Register

**Opcode:** `0x03`  
**Category:** Data Movement

**Description:**  
Loads a 16-bit value from memory into one of the 16 general-purpose registers (R0–R15).

**Operands:**  
- **Register (1 byte):** Index from 0–15 representing R0–R15.  
- **Memory Address (2 byte):** Memory address from which to load the value.

**Instruction Length:**  
4 bytes  
(1 opcode + 1 register + 2 memory bytes)

**Binary Format:**  
[03] [register] [address_hi] [address_lo]

**Execution Semantics:**  
```R[register] ← MEM[address]]```

**Flags Affected:**  
- **Zero (Z):** Set if the copied value equals 0.  
- **Carry (C):** Unaffected.

**Example Encoding:**  
`03 04 12 34 `  
Copies the value from memory addres 0x1234 into R4.

### 5.4 - Store from Register

**Opcode:** `0x04`  
**Category:** Data Movement

**Description:**  
Loads a 16-bit value from a register (R0-R15) into memory.

**Operands:**  
- **Address (2 bytes):** Memory address from which to load the value.
- **Register (1 byte):** Index from 0–15 representing R0–R15.

**Instruction Length:**  
4 bytes  
(1 opcode + 2 byte memory address + 1 register)

**Binary Format:**  
[04] [address_hi] [address_lo] [register]

**Execution Semantics:**  
```MEM[address] ← R[register]```

**Flags Affected:**  
- **Zero (Z):** Unaffected.
- **Carry (C):** Unaffected.

**Example Encoding:**  
`04 12 34 07`  
Copies the value of R7 into memory address 0x1234.

### **5.5 – Add Register to Register**

**Opcode:** `0x05`
**Category:** Arithmetic

**Description:**
Adds the value of a source register to a destination register and stores the result in the destination register.

**Operands:**

- **Destination Register (1 byte):** Index from 0–15 representing R0–R15.
- **Source Register (1 byte):** Index from 0–15 representing R0–R15.

**Instruction Length:**
3 bytes
(1 opcode + 1 destination register + 1 source register)

**Binary Format:**
[05] [dest_reg] [src_reg]

**Execution Semantics:**
Register `dest_reg` is updated with the sum of its current value and the source register:
`R[dest_reg] ← R[dest_reg] + R[src_reg]`

**Flags Affected:**

- **Zero (Z):** Set if the result equals 0.
- **Carry (C):** Set if an arithmetic overflow occurs.

**Example Encoding:**
`05 02 03`
Adds the value of R3 to R2 and stores the result in R2.

### **5.6 – Subtract Register from Register**

**Opcode:** `0x06`
**Category:** Arithmetic

**Description:**
Subtracts the value of a source register from a destination register and stores the result in the destination register.

**Operands:**

- **Destination Register (1 byte):** Index from 0–15 representing R0–R15.
- **Source Register (1 byte):** Index from 0–15 representing R0–R15.

**Instruction Length:**
3 bytes
(1 opcode + 1 destination register + 1 source register)

**Binary Format:**
[06] [dest_reg] [src_reg]

**Execution Semantics:**
Register `dest_reg` is updated with the result of the subtraction:
`R[dest_reg] ← R[dest_reg] − R[src_reg]`

**Flags Affected:**

- **Zero (Z):** Set if the result equals 0.
- **Carry (C):** Set if a borrow occurs.

**Example Encoding:**
`06 01 04`
Subtracts the value of R4 from R1 and stores the result in R1.

### **5.7 – Compare Registers**

**Opcode:** `0x07`
**Category:** Comparison

**Description:**
Compares two registers by subtracting the second register from the first without storing the result.

**Operands:**

- **Register A (1 byte):** Index from 0–15 representing R0–R15.
- **Register B (1 byte):** Index from 0–15 representing R0–R15.

**Instruction Length:**
3 bytes
(1 opcode + 1 register + 1 register)

**Binary Format:**
[07] [reg_a] [reg_b]

**Execution Semantics:**
A temporary subtraction is performed:
`temp ← R[reg_a] − R[reg_b]`
No registers are modified.

**Flags Affected:**

- **Zero (Z):** Set if the values of the two registers are equal.
- **Carry (C):** Set if a borrow occurs.

**Example Encoding:**
`07 02 05`
Compares the value in R2 with the value in R5.

### **5.8 – Unconditional Jump**

**Opcode:** `0x08`
**Category:** Control Flow

**Description:**
Sets the program counter to a specified absolute memory address.

**Operands:**

- **Address (2 bytes, big-endian):** Target address to jump to.

**Instruction Length:**
3 bytes
(1 opcode + 2 address bytes)

**Binary Format:**
[08] [addr_hi] [addr_lo]

**Execution Semantics:**
The program counter is updated to the specified address:
`PC ← address`

**Flags Affected:**

- **Zero (Z):** Unaffected.
- **Carry (C):** Unaffected.

**Example Encoding:**
`08 01 00`
Jumps execution to memory address `0x0100`.

### **5.9 – Jump if Zero**

**Opcode:** `0x09`
**Category:** Control Flow

**Description:**
Jumps to a specified address if the Zero (Z) flag is set.

**Operands:**

- **Address (2 bytes, big-endian):** Target address to jump to.

**Instruction Length:**
3 bytes
(1 opcode + 2 address bytes)

**Binary Format:**
[09] [addr_hi] [addr_lo]

**Execution Semantics:**
If the Zero flag is set, the program counter is updated:
`if Z == 1 then PC ← address`

**Flags Affected:**

- **Zero (Z):** Unaffected.
- **Carry (C):** Unaffected.

**Example Encoding:**
`09 00 20`
Jumps to address `0x0020` if the Zero flag is set.

### **5.10 – Jump if Not Zero**

**Opcode:** `0x0A`
**Category:** Control Flow

**Description:**
Jumps to a specified address if the Zero (Z) flag is not set.

**Operands:**

- **Address (2 bytes, big-endian):** Target address to jump to.

**Instruction Length:**
3 bytes
(1 opcode + 2 address bytes)

**Binary Format:**
[0A] [addr_hi] [addr_lo]

**Execution Semantics:**
If the Zero flag is not set, the program counter is updated:
`if Z == 0 then PC ← address`

**Flags Affected:**

- **Zero (Z):** Unaffected.
- **Carry (C):** Unaffected.

**Example Encoding:**
`0A 00 10`
Jumps to address `0x0010` if the Zero flag is not set.

### **5.11 – Halt Execution**

**Opcode:** `0xFF`
**Category:** System

**Description:**
Stops execution of the current program.

**Operands:**
None.

**Instruction Length:**
1 byte
(1 opcode)

**Binary Format:**
[FF]

**Execution Semantics:**
The virtual machine stops execution immediately.

**Flags Affected:**

- **Zero (Z):** Unaffected.
- **Carry (C):** Unaffected.

**Example Encoding:**
`FF`
Halts program execution.

## 6. Summary Table

| Instruction Name           | Opcode | Operands                    | Instruction Length |
| -------------------------- | ------ | --------------------------- | ------------------ |
| Move Immediate to Register | 0x01   | reg (1B), imm16 (2B)        | 4 bytes            |
| Move Register to Register  | 0x02   | dest_reg (1B), src_reg (1B) | 3 bytes            |
| Load to Register           | 0x03   | reg (1B), address (2B)      | 4 bytes            |
| Store from Register        | 0x04   | address (2B), reg (1B)      | 4 bytes            |
| Add Register to Register   | 0x05   | dest_reg (1B), src_reg (1B) | 3 bytes            |
| Subtract Register          | 0x06   | dest_reg (1B), src_reg (1B) | 3 bytes            |
| Compare Registers          | 0x07   | reg_a (1B), reg_b (1B)      | 3 bytes            |
| Unconditional Jump         | 0x08   | address (2B)                | 3 bytes            |
| Jump if Zero               | 0x09   | address (2B)                | 3 bytes            |
| Jump if Not Zero           | 0x0A   | address (2B)                | 3 bytes            |
| Halt Execution             | 0xFF   | none                        | 1 byte             |
