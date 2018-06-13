# Architecture of Jello

## Introduction

The design of the virtual architecture used for Rusty Jello was based on a combination of simple stack based machines I had previously come into contact with, plus a wish to somewhat simplify the instructions involved (as I was learning Rust while designing it). As such the design was quite ad hoc and is, in fact, still ongoing. What follows is an overview of the architecture, including the instructions and assembly.

## Machine

### Components

The machine has:

  - a 16-bit accumulator
  - a 16-bit instruction pointer
  - 2 16-bit general purpose registers (R0, R1)
  - 2 16-bit special purpose registers (R2, R3)
  - a 16 × 16-bit general purpose stack
  - a 16 × 16-bit instruction pointer stack
  - 16 flags (halt [15], carry [0], overflow [1], test [2])
  - 64KiB of memory

**Notes**:
  - Of the 16 flags currently only 4 are used.
  - Furthermore both special purpose registers currently act simply as general purpose registers, however they're behaviour will change in the future so use is discouraged.
  - R0 is frequently used in test and set instructions which interact with registers.

### Data Transfer

Data can be transferred between the following components

  - accumulator <-> registers
  - accumulator <-> stack
  - memory <-> stack
  - instruction decoder <- stack
  - instruction decoder <- memory
  - instruction decoder -> accumulator
  - ALU <- stack
  - ALU -> accumulator
  - instruction pointer <-> instruction pointer stack
  - stack -> instruction pointer
  - instruction decoder <-> instruction pointer
  - instruction decoder -> debug output
  - stack -> debug output

Data transfer to registers is somewhat slower than data transfer to the stack. Data transfer to memory is significantly slower than data transfer to the stack. The accumulator is no slower to access than the stack as it is effectively a special slot on top of the stack. The ALU can only output to the accumulator directly so any instructions which output to the stack will be mildly slower (as they in effect are pushed to the stack as part of the instruction).

## Instructions

### Overview

Jello instructions are simple, only ever having 0 or 1 immediate operands, and rarely using more than 2 items from the stack (the glaring excepting being rotation instructions for obvious reasons). The instruction set is not complete, with new instructions added as features are implemented. The initial instruction set is based on the instruction set outlined by Dr Crispin-Bailey in the FESC module at the University of York, with additions and modifications based on existing instructions sets and further learning through other modules (such as SYST/IMPL).

### Instruction Set

Index of Terms:

  - ACC = Accumulator
  - HALT, CARRY, OVERFLOW, TEST = Flags
  - R0, R1, R2, R3 = Registers
  - STACK/stack = General Purpose Stack
  - IP-STACK/ip-stack = Instruction Pointer Stack
  - IP = Instruction Pointer
  - MEM = Memory
  - UD = Use Discouraged
  - WIP = Work In Progress
  - FCP = Future Change Probable
  - DEP = Deprecated

|Mnemonic|Op-Code|Operands|Description|
|:-:|:-:|:-:|:-:|
|NOOP|0x00|N/A|No operation|
|UNDEFINED|0x01|N/A|UNDEFINED|
|UNDEFINED|0x02|N/A|UNDEFINED|
|UNDEFINED|0x03|N/A|UNDEFINED|
|UNDEFINED|0x04|N/A|UNDEFINED|
|UNDEFINED|0x05|N/A|UNDEFINED|
|UNDEFINED|0x06|N/A|UNDEFINED|
|UNDEFINED|0x07|N/A|UNDEFINED|
|HALT|0x08|N/A|Sets HALT flag|
|UNDEFINED|0x09|N/A|UNDEFINED|
|UNDEFINED|0x0a|N/A|UNDEFINED|
|UNDEFINED|0x0b|N/A|UNDEFINED|
|UNDEFINED|0x0c|N/A|UNDEFINED|
|UNDEFINED|0x0d|N/A|UNDEFINED|
|UNDEFINED|0x0e|N/A|UNDEFINED|
|UNDEFINED|0x0f|N/A|UNDEFINED|
|LRI|0x10|1 × immediate 16-bit value|Loads operand into ACC|
|LR0|0x11|1 × register 16-bit value|Loads R0 into ACC|
|LR1|0x12|1 × register 16-bit value|Loads R1 into ACC|
|LR2|0x13|1 × register 16-bit value|Loads R2 into ACC **UD-FCP**|
|LR3|0x14|1 × register 16-bit value|Loads R3 into ACC **UD-FCP**|
|SR0|0x15|1 × accumulator 16-bit value|Store ACC in R0|
|SR1|0x16|1 × accumulator 16-bit value|Store ACC in R1|
|SR2|0x17|1 × accumulator 16-bit value|Store ACC in R2|
|SR3|0x18|1 × accumulator 16-bit value|Store ACC in R3|
|ZERO|0x19|N/A|Sets ACC to 0x0000|
|LRS|0x1a|1 × immediate 16-bit address|Loads 1 byte from memory specified by operand into low byte of ACC|
|UNDEFINED|0x1b|N/A|UNDEFINED|
|UNDEFINED|0x1c|N/A|UNDEFINED|
|UNDEFINED|0x1d|N/A|UNDEFINED|
|UNDEFINED|0x1e|N/A|UNDEFINED|
|UNDEFINED|0x1f|N/A|UNDEFINED|
|ADD|0x20|2 × 16-bit values popped off stack|Adds operands and stores result in ACC|
|ADDC|0x21|2 × 16-bit values popped off stack|Adds operands with carry in and stores result in ACC|
|SUB|0x22|2 × 16-bit values popped off stack|Subtracts operand 2 from operand 1 and stores result in ACC|
|SUBC|0x23|2 × 16-bit values popped off stack|Subtracts operand 2 from operand 1 with carry in and stores result in ACC|
|NEG|0x24|1 × 16-bit value popped off stack|Negates operand and stores result in ACC|
|MUL|0x25|2 × 16-bit values popped off stack|Multiplies operands together and stores result in ACC|
|DIV|0x26|2 × 16-bit values popped off stack|Divides input 1 by input 2 and stores result in ACC|
|ADDI|0x27|1 × immediate 16-bit value, 1 × 16-bit value popped off stack|Adds operands and stores the result in ACC|
|SUBI|0x28|1 × immediate 16-bit value, 1 × 16-bit value popped off stack|Subtracts immediate operand from stack operand and stores the result in ACC|
|UNDEFINED|0x29|N/A|UNDEFINED|
|UNDEFINED|0x2a|N/A|UNDEFINED|
|UNDEFINED|0x2b|N/A|UNDEFINED|
|UNDEFINED|0x2c|N/A|UNDEFINED|
|UNDEFINED|0x2d|N/A|UNDEFINED|
|UNDEFINED|0x2e|N/A|UNDEFINED|
|UNDEFINED|0x2f|N/A|UNDEFINED|
|PUSH|0x30|1 × accumulator 16-bit value|Pushes ACC onto stack|
|POP|0x31|1 × 16-bit value popped off stack|Pops the top of stack off into ACC|
|SWAP|0x31|N/A|Swaps the top 2 items of the stack|
|PEEK|0x33|1 × stack 16-bit value|Stores the top of the stack in ACC|
|SPILL|0x34|N/A|Drops the second item on the stack|
|DROP|0x35|N/A|Drops the top of the stack|
|UNDER|0x36|1 × accumulator 16-bit value|Pushes ACC into the second slot of the stack|
|ROTCW|0x37|N/A|Rotates the top 3 items of the stack clockwise|
|ROTAC|0x38|N/A|Rotates the top 3 items of the stack anti-clockwise|
|DUP|0x39|1 × stack 16-bit value|Copies the top of the stack and pushes it onto the stack|
|PUSHI|0x3a|1 × immediate 16-bit value|Pushes operand onto the stack|
|ROTCW4|0x3b|N/A|Rotates the top 4 items of the stack clockwise|
|ROTAC4|0x3c|N/A|Rotates the top 4 items of the stack anti-clockwise|
|ROTCW5|0x3d|N/A|Rotates the top 5 items of the stack clockwise|
|ROTAC5|0x3e|N/A|Rotates the top 5 items of the stack anti-clockwise|
|UNDEFINED|0x3f|N/A|UNDEFINED|
|JMPI|0x40|1 × immediate 16-bit address|Unconditional jump to immediate address operand|
|JMPIG|0x41|1 × immediate 16-bit address, 2 × stack 16-bit values|Jump to immediate address operand if top of stack greater than second item on stack|
|JMPIL|0x42|1 × immediate 16-bit address, 2 × stack 16-bit values|Jump to immediate address operand if top of stack less than second item on stack|
|JMPIE|0x43|1 × immediate 16-bit address, 2 × stack 16-bit values|Jump to immediate address operand if top of stack equal to second item on stack|
|JMPIT|0x44|1 × immediate 16-bit address|Jump to immediate address operand if TEST flag is set|
|JMP|0x45|1 × 16-bit address popped off the stack|Unconditional jump to address operand|
|JMPGT|0x46|1 × 16-bit address popped off the stack, 2 × stack 16-bit values|Jump to address operand if top of stack (after address is popped) greater than second item on stack|
|JMPLT|0x47|1 × 16-bit address popped off the stack, 2 × stack 16-bit values|Jump to address operand if top of stack (after address is popped) less than second item on stack|
|JMPEQ|0x48|1 × 16-bit address popped off the stack, 2 × stack 16-bit values|Jump to address operand if top of stack (after address is popped) equal to second item on stack|
|JMPT|0x49|1 × 16-bit address popped off the stack|Jump to address operand if TEST flag is set|
|PUSHP|0x4a|N/A|Push current instruction pointer onto the ip-stack|
|POPP|0x4b|N/A|Pop item off ip-stack and store in instruction pointer, but does not increment by 1|
|CALL|0x4c|1 × 16-bit address popped off the stack|Push current instruction pointer onto ip-stack then jump to address operand|
|RET|0x4d|N/A|Pop item off ip-stack and store in instruction pointer, then increment by 1|
|CALLI|0x4e|1 × immediate 16-bit address|Push current instruction pointer onto ip-stack then jump to immediate address operand|
|SKIP|0x4f|N/A|Skip next instruction|
|LOAD|0x50|1 × 16-bit address popped off the stack|Load 16-bit value from address operand address and push onto stack|
|STORE|0x51|1 × 16-bit address popped off the stack, 1 × 16-bit value popped off the stack|Pop the top item off the stack (item at the top after address has been popped) and store at address operand address|
|LOADI|0x52|1 × immediate 16-bit address|Load 16-bit value from address operand address and push onto stack|
|STOREI|0x53|1 × immediate 16-bit address, 1 × 16-bit value popped off the stack|Pop the top item off the stack and store at immediate address operand address|
|UNDEFINED|0x54|N/A|UNDEFINED|
|UNDEFINED|0x55|N/A|UNDEFINED|
|UNDEFINED|0x56|N/A|UNDEFINED|
|UNDEFINED|0x57|N/A|UNDEFINED|
|UNDEFINED|0x58|N/A|UNDEFINED|
|UNDEFINED|0x59|N/A|UNDEFINED|
|UNDEFINED|0x5a|N/A|UNDEFINED|
|UNDEFINED|0x5b|N/A|UNDEFINED|
|UNDEFINED|0x5c|N/A|UNDEFINED|
|UNDEFINED|0x5d|N/A|UNDEFINED|
|UNDEFINED|0x5e|N/A|UNDEFINED|
|UNDEFINED|0x5f|N/A|UNDEFINED|
|OR|0x60|2 × 16-bit values popped off stack|Bitwise OR operands and store in ACC|
|AND|0x61|2 × 16-bit values popped off stack|Bitwise AND operands and store in ACC|
|XOR|0x62|2 × 16-bit values popped off stack|Bitwise XOR operands and store in ACC|
|NAND|0x63|2 × 16-bit values popped off stack|Bitwise NAND operands and store in ACC|
|NOR|0x64|2 × 16-bit values popped off stack|Bitwise NOR operands and store in ACC|
|NOT|0x65|1 × 16-bit value popped off stack|Bitwise NOT operand and store in ACC|
|LSFT0|0x66|1 × 16-bit value popped off stack|Left shift operand, padding with zeros, and store in ACC|
|RSFT0|0x67|1 × 16-bit value popped off stack|Right shift operand, padding with zeros, and store in ACC|
|LSFT1|0x68|1 × 16-bit value popped off stack|Left shift operand, padding with ones, and store in ACC|
|RSFT1|0x69|1 × 16-bit value popped off stack|Right shift operand, padding with ones, and store in ACC|
|ORI|0x6a|1 × immediate 16-bit value, 1 × 16-bit value popped off stack|Bitwise OR operands and store in ACC|
|ANDI|0x6b|1 × immediate 16-bit value, 1 × 16-bit value popped off stack|Bitwise AND operands and store in ACC|
|XORI|0x6c|1 × immediate 16-bit value, 1 × 16-bit value popped off stack|Bitwise XOR operands and store in ACC|
|NANDI|0x6d|1 × immediate 16-bit value, 1 × 16-bit value popped off stack|Bitwise NAND operands and store in ACC|
|NORI|0x6e|1 × immediate 16-bit value, 1 × 16-bit value popped off stack|Bitwise NOR operands and store in ACC|
|UNDEFINED|0x6f|N/A|UNDEFINED|
|INC|0x70|N/A|UNDEFINED|
|INC2|0x71|N/A|UNDEFINED|
|INC3|0x72|N/A|UNDEFINED|
|INC4|0x73|N/A|UNDEFINED|
|DEC|0x74|N/A|UNDEFINED|
|DEC2|0x75|N/A|UNDEFINED|
|DEC3|0x76|N/A|UNDEFINED|
|DEC4|0x77|N/A|UNDEFINED|
|INCP|0x78|N/A|UNDEFINED|
|INC2P|0x79|N/A|UNDEFINED|
|INC3P|0x7a|N/A|UNDEFINED|
|INC4P|0x7b|N/A|UNDEFINED|
|DECP|0x7c|N/A|UNDEFINED|
|DEC2P|0x7d|N/A|UNDEFINED|
|DEC3P|0x7e|N/A|UNDEFINED|
|DEC4P|0x7f|N/A|UNDEFINED|
|TEST|0x80|N/A|UNDEFINED|
|TESTC|0x81|N/A|UNDEFINED|
|TESTO|0x82|N/A|UNDEFINED|
|UNDEFINED|0x83|N/A|UNDEFINED|
|UNDEFINED|0x84|N/A|UNDEFINED|
|UNDEFINED|0x85|N/A|UNDEFINED|
|UNDEFINED|0x86|N/A|UNDEFINED|
|UNDEFINED|0x87|N/A|UNDEFINED|
|TSAST|0x88|1 × 16-bit value popped off stack|Tests if top of stack is non-zero and pushes 0x0001 onto stack|
|TSAINC|0x89|1 × 16-bit value popped off stack|Tests if top of stack is non-zero then pushes top of stack + 1 onto stack|
|TSADEC|0x8a|1 × 16-bit value popped off stack|Tests if top of stack is non-zero then pushes top of stack - 1 onto stack|
|TSASTR|0x8b|1 × 16-bit register value|Tests if R0 is non-zero then stores 0x0001 in R0|
|TSAINR|0x8c|1 × 16-bit register value|Tests if R0 is non-zero then stores R0 + 1 in R0|
|TSADER|0x8d|1 × 16-bit register value|Tests if R0 is non-zero then stores R0 - 1 in R0|
|UNDEFINED|0x8e|N/A|UNDEFINED|
|UNDEFINED|0x8f|N/A|UNDEFINED|
|LSFTB|0x90|1 × 16-bit value popped off stack|Left shifts operand by a byte and stores in ACC|
|RSFTB|0x91|1 × 16-bit value popped off stack|Right shifts operand by a byte and stores in ACC|
|UNDEFINED|0x92|N/A|UNDEFINED|
|UNDEFINED|0x93|N/A|UNDEFINED|
|UNDEFINED|0x94|N/A|UNDEFINED|
|UNDEFINED|0x95|N/A|UNDEFINED|
|UNDEFINED|0x96|N/A|UNDEFINED|
|UNDEFINED|0x97|N/A|UNDEFINED|
|UNDEFINED|0x98|N/A|UNDEFINED|
|UNDEFINED|0x99|N/A|UNDEFINED|
|UNDEFINED|0x9a|N/A|UNDEFINED|
|UNDEFINED|0x9b|N/A|UNDEFINED|
|UNDEFINED|0x9c|N/A|UNDEFINED|
|UNDEFINED|0x9d|N/A|UNDEFINED|
|UNDEFINED|0x9e|N/A|UNDEFINED|
|UNDEFINED|0x9f|N/A|UNDEFINED|
|UNDEFINED|0xa0|N/A|UNDEFINED|
|UNDEFINED|0xa1|N/A|UNDEFINED|
|UNDEFINED|0xa2|N/A|UNDEFINED|
|UNDEFINED|0xa3|N/A|UNDEFINED|
|UNDEFINED|0xa4|N/A|UNDEFINED|
|UNDEFINED|0xa5|N/A|UNDEFINED|
|UNDEFINED|0xa6|N/A|UNDEFINED|
|UNDEFINED|0xa7|N/A|UNDEFINED|
|UNDEFINED|0xa8|N/A|UNDEFINED|
|UNDEFINED|0xa9|N/A|UNDEFINED|
|UNDEFINED|0xaa|N/A|UNDEFINED|
|UNDEFINED|0xab|N/A|UNDEFINED|
|UNDEFINED|0xac|N/A|UNDEFINED|
|UNDEFINED|0xad|N/A|UNDEFINED|
|UNDEFINED|0xae|N/A|UNDEFINED|
|UNDEFINED|0xaf|N/A|UNDEFINED|
|UNDEFINED|0xb0|N/A|UNDEFINED|
|UNDEFINED|0xb1|N/A|UNDEFINED|
|UNDEFINED|0xb2|N/A|UNDEFINED|
|UNDEFINED|0xb3|N/A|UNDEFINED|
|UNDEFINED|0xb4|N/A|UNDEFINED|
|UNDEFINED|0xb5|N/A|UNDEFINED|
|UNDEFINED|0xb6|N/A|UNDEFINED|
|UNDEFINED|0xb7|N/A|UNDEFINED|
|UNDEFINED|0xb8|N/A|UNDEFINED|
|UNDEFINED|0xb9|N/A|UNDEFINED|
|UNDEFINED|0xba|N/A|UNDEFINED|
|UNDEFINED|0xbb|N/A|UNDEFINED|
|UNDEFINED|0xbc|N/A|UNDEFINED|
|UNDEFINED|0xbd|N/A|UNDEFINED|
|UNDEFINED|0xbe|N/A|UNDEFINED|
|UNDEFINED|0xbf|N/A|UNDEFINED|
|UNDEFINED|0xc0|N/A|UNDEFINED|
|UNDEFINED|0xc1|N/A|UNDEFINED|
|UNDEFINED|0xc2|N/A|UNDEFINED|
|UNDEFINED|0xc3|N/A|UNDEFINED|
|UNDEFINED|0xc4|N/A|UNDEFINED|
|UNDEFINED|0xc5|N/A|UNDEFINED|
|UNDEFINED|0xc6|N/A|UNDEFINED|
|UNDEFINED|0xc7|N/A|UNDEFINED|
|UNDEFINED|0xc8|N/A|UNDEFINED|
|UNDEFINED|0xc9|N/A|UNDEFINED|
|UNDEFINED|0xca|N/A|UNDEFINED|
|UNDEFINED|0xcb|N/A|UNDEFINED|
|UNDEFINED|0xcc|N/A|UNDEFINED|
|UNDEFINED|0xcd|N/A|UNDEFINED|
|UNDEFINED|0xce|N/A|UNDEFINED|
|UNDEFINED|0xcf|N/A|UNDEFINED|
|UNDEFINED|0xd0|N/A|UNDEFINED|
|UNDEFINED|0xd1|N/A|UNDEFINED|
|UNDEFINED|0xd2|N/A|UNDEFINED|
|UNDEFINED|0xd3|N/A|UNDEFINED|
|UNDEFINED|0xd4|N/A|UNDEFINED|
|UNDEFINED|0xd5|N/A|UNDEFINED|
|UNDEFINED|0xd6|N/A|UNDEFINED|
|UNDEFINED|0xd7|N/A|UNDEFINED|
|UNDEFINED|0xd8|N/A|UNDEFINED|
|UNDEFINED|0xd9|N/A|UNDEFINED|
|UNDEFINED|0xda|N/A|UNDEFINED|
|UNDEFINED|0xdb|N/A|UNDEFINED|
|UNDEFINED|0xdc|N/A|UNDEFINED|
|UNDEFINED|0xdd|N/A|UNDEFINED|
|UNDEFINED|0xde|N/A|UNDEFINED|
|UNDEFINED|0xdf|N/A|UNDEFINED|
|UNDEFINED|0xe0|N/A|UNDEFINED|
|UNDEFINED|0xe1|N/A|UNDEFINED|
|UNDEFINED|0xe2|N/A|UNDEFINED|
|UNDEFINED|0xe3|N/A|UNDEFINED|
|UNDEFINED|0xe4|N/A|UNDEFINED|
|UNDEFINED|0xe5|N/A|UNDEFINED|
|UNDEFINED|0xe6|N/A|UNDEFINED|
|UNDEFINED|0xe7|N/A|UNDEFINED|
|UNDEFINED|0xe8|N/A|UNDEFINED|
|UNDEFINED|0xe9|N/A|UNDEFINED|
|UNDEFINED|0xea|N/A|UNDEFINED|
|UNDEFINED|0xeb|N/A|UNDEFINED|
|UNDEFINED|0xec|N/A|UNDEFINED|
|UNDEFINED|0xed|N/A|UNDEFINED|
|UNDEFINED|0xee|N/A|UNDEFINED|
|UNDEFINED|0xef|N/A|UNDEFINED|
|PRN|0xf0|1 × 16-bit value popped off stack|Outputs low byte of operand to debug output (pseudo RS232)|
|PRNI|0xf1|1 × immediate 8-bit value|Outputs operand to debug output (pseudo RS232)|
|PRN2|0xf2|1 × 16-bit value popped off stack|Outputs both bytes of operand (low byte first) to debug output (pseudo RS232)|
|PRN2I|0xf3|2 × immediate 8-bit value|Outputs operands to debug output (pseudo RS232)|
|DUMP8|0xf4|1 × 16-bit value popped off stack|Formats low byte of operand as hex and outputs to debug output (pseudo RS232)|
|DUMP16|0xf5|1 × 16-bit value popped off stack|Formats operand as hex and outputs to debug output (pseudo RS232)|
|UNDEFINED|0xf6|N/A|UNDEFINED|
|UNDEFINED|0xf7|N/A|UNDEFINED|
|UNDEFINED|0xf8|N/A|UNDEFINED|
|UNDEFINED|0xf9|N/A|UNDEFINED|
|UNDEFINED|0xfa|N/A|UNDEFINED|
|UNDEFINED|0xfb|N/A|UNDEFINED|
|UNDEFINED|0xfc|N/A|UNDEFINED|
|UNDEFINED|0xfd|N/A|UNDEFINED|
|UNDEFINED|0xfe|N/A|UNDEFINED|
|UNDEFINED|0xff|N/A|UNDEFINED|
