//! Module for storing instruction definitions
use machine::Machine;

#[derive(Clone)]
pub struct Instruction<'x> {
  pub inst: &'x str,
  pub num_args: u8,
  pub op_code: u8,
  pub run: &'x Fn(&mut Machine),
  pub bytes_per_arg: u8,
  pub clock_cycles: usize,
}

pub fn find_inst_by_opcode(op_code: &u8) -> Option<Instruction> {
  for index in 0..INSTRUCTION_COUNT {
    if INSTRUCTIONS[index].op_code == *op_code {
      return Some(INSTRUCTIONS[index].clone());
    }
  }
  return None;
}

pub fn find_inst_by_name(name: &str) -> Option<Instruction> {
  for index in 0..INSTRUCTION_COUNT {
    if INSTRUCTIONS[index].inst == name.to_uppercase() {
      return Some(INSTRUCTIONS[index].clone());
    }
  }
  return None;
}

pub const INSTRUCTION_COUNT: usize = 105;

pub const INSTRUCTIONS: [Instruction; INSTRUCTION_COUNT] = [
  Instruction {
    inst: "NOOP",
    num_args: 0,
    op_code: 0x0,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "HALT",
    num_args: 0,
    op_code: 0x08,
    run: &|machine: &mut Machine| {
      machine.flags.halt = true;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "LRI",
    num_args: 1,
    op_code: 0x10,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      machine.accumulator = part1 | part2;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "LR0",
    num_args: 0,
    op_code: 0x11,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[0];
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "LR1",
    num_args: 0,
    op_code: 0x12,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[1];
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "LR2",
    num_args: 0,
    op_code: 0x13,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[2];
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "LR3",
    num_args: 0,
    op_code: 0x14,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[3];
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "SR0",
    num_args: 0,
    op_code: 0x15,
    run: &|machine: &mut Machine| {
      machine.registers[0] = machine.accumulator;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "SR1",
    num_args: 0,
    op_code: 0x16,
    run: &|machine: &mut Machine| {
      machine.registers[1] = machine.accumulator;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "SR2",
    num_args: 0,
    op_code: 0x17,
    run: &|machine: &mut Machine| {
      machine.registers[2] = machine.accumulator;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "SR3",
    num_args: 0,
    op_code: 0x18,
    run: &|machine: &mut Machine| {
      machine.registers[3] = machine.accumulator;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "ZERO",
    num_args: 0,
    op_code: 0x19,
    run: &|machine: &mut Machine| {
      machine.accumulator = 0;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "LRS",
    num_args: 1,
    op_code: 0x1a,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      machine.accumulator = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 1,
    clock_cycles: 2,
  },
  Instruction {
    inst: "ADD",
    num_args: 0,
    op_code: 0x20,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      let input2: u16 = machine.stack.pop();
      let mut result: u32 = input1 as u32 + input2 as u32;
      let carry: bool;
      if result > 65535 {
        carry = true;
        result = result % 65536;
      } else {
        carry = false;
      }
      machine.accumulator = result as u16;
      machine.flags.carry = carry;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "ADDC",
    num_args: 0,
    op_code: 0x21,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      let input2: u16 = machine.stack.pop();
      let mut result: u32 = input1 as u32 + input2 as u32 + if machine.flags.carry { 1 } else { 0 };
      let carry: bool;
      if result > 65535 {
        carry = true;
        result = result % 65536;
      } else {
        carry = false;
      }
      machine.accumulator = result as u16;
      machine.flags.carry = carry;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "SUB",
    num_args: 0,
    op_code: 0x22,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      let input2: u16 = machine.stack.pop();
      let mut result: i32 = input1 as i32 - input2 as i32;
      let carry: bool;
      if result < 0 {
        carry = true;
        result = 65536 + result;
      } else {
        carry = false;
      }
      machine.accumulator = result as u16;
      machine.flags.carry = carry;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "SUBC",
    num_args: 0,
    op_code: 0x23,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      let input2: u16 = machine.stack.pop();
      let mut result: i32 =
        input1 as i32 - input2 as i32 + if machine.flags.carry { -1 } else { 0 };
      let carry: bool;
      if result < 0 {
        carry = true;
        result = 65536 + result;
      } else {
        carry = false;
      }
      machine.accumulator = result as u16;
      machine.flags.carry = carry;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "NEG",
    num_args: 0,
    op_code: 0x24,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      machine.accumulator = input1 ^ 65535;
      machine.accumulator = (machine.accumulator as u32 + 1u32) as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "MUL",
    num_args: 0,
    op_code: 0x25,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      let input2: u16 = machine.stack.pop();
      let mut result: u64 = input1 as u64 * input2 as u64;
      let overflow: bool;
      if result > 65535 {
        overflow = true;
        result = 65535;
      } else {
        overflow = false;
      }
      machine.accumulator = result as u16;
      machine.flags.overflow = overflow;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "DIV",
    num_args: 0,
    op_code: 0x26,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      let input2: u16 = machine.stack.pop();
      let result: u16 = input1 as u16 / input2 as u16;
      machine.accumulator = result as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 3,
  },
  Instruction {
    inst: "ADDI",
    num_args: 1,
    op_code: 0x27,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;
      let mut result: u32 = input1 as u32 + immediate as u32;
      let carry: bool;
      if result > 65535 {
        carry = true;
        result = result % 65536;
      } else {
        carry = false;
      }
      machine.accumulator = result as u16;
      machine.flags.carry = carry;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "SUBI",
    num_args: 1,
    op_code: 0x28,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;
      let mut result: i32 = input1 as i32 - immediate as i32;
      let carry: bool;
      if result < 0 {
        carry = true;
        result = 65536 + result;
      } else {
        carry = false;
      }
      machine.accumulator = result as u16;
      machine.flags.carry = carry;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "PUSH",
    num_args: 0,
    op_code: 0x30,
    run: &|machine: &mut Machine| {
      machine.stack.push(machine.accumulator);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "POP",
    num_args: 0,
    op_code: 0x31,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.stack.pop();
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "SWAP",
    num_args: 0,
    op_code: 0x32,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.stack.push(first);
      machine.stack.push(second);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "PEAK",
    num_args: 0,
    op_code: 0x33,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.stack.push(first);
      machine.accumulator = first;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "SPILL",
    num_args: 0,
    op_code: 0x34,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.stack.pop();
      machine.stack.push(first);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "DROP",
    num_args: 0,
    op_code: 0x35,
    run: &|machine: &mut Machine| {
      machine.stack.pop();
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "UNDER",
    num_args: 0,
    op_code: 0x36,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.stack.push(machine.accumulator);
      machine.stack.push(first);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "ROTCW",
    num_args: 0,
    op_code: 0x37,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      let third: u16 = machine.stack.pop();
      machine.stack.push(second);
      machine.stack.push(first);
      machine.stack.push(third);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "ROTAC",
    num_args: 0,
    op_code: 0x38,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      let third: u16 = machine.stack.pop();
      machine.stack.push(first);
      machine.stack.push(third);
      machine.stack.push(second);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "DUP",
    num_args: 0,
    op_code: 0x39,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.stack.push(first);
      machine.stack.push(first);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "PUSHI",
    num_args: 1,
    op_code: 0x3a,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;
      machine.stack.push(immediate);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "ROTCW4",
    num_args: 0,
    op_code: 0x3b,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      let third: u16 = machine.stack.pop();
      let fourth: u16 = machine.stack.pop();
      machine.stack.push(third);
      machine.stack.push(second);
      machine.stack.push(first);
      machine.stack.push(fourth);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "ROTAC4",
    num_args: 0,
    op_code: 0x3c,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      let third: u16 = machine.stack.pop();
      let fourth: u16 = machine.stack.pop();
      machine.stack.push(first);
      machine.stack.push(fourth);
      machine.stack.push(third);
      machine.stack.push(second);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "ROTCW5",
    num_args: 0,
    op_code: 0x3d,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      let third: u16 = machine.stack.pop();
      let fourth: u16 = machine.stack.pop();
      let fifth: u16 = machine.stack.pop();
      machine.stack.push(first);
      machine.stack.push(fifth);
      machine.stack.push(fourth);
      machine.stack.push(third);
      machine.stack.push(second);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "ROTAC5",
    num_args: 0,
    op_code: 0x3e,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      let third: u16 = machine.stack.pop();
      let fourth: u16 = machine.stack.pop();
      let fifth: u16 = machine.stack.pop();
      machine.stack.push(fourth);
      machine.stack.push(third);
      machine.stack.push(second);
      machine.stack.push(first);
      machine.stack.push(fifth);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "JMPI",
    num_args: 1,
    op_code: 0x40,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;
      machine.instruction_pointer = immediate;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "JMPIG",
    num_args: 1,
    op_code: 0x41,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.stack.push(second);
      machine.stack.push(first);
      if first > second {
        machine.instruction_pointer = immediate;
      } else {
        machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      }
    },
    bytes_per_arg: 2,
    clock_cycles: 4,
  },
  Instruction {
    inst: "JMPIL",
    num_args: 1,
    op_code: 0x42,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.stack.push(second);
      machine.stack.push(first);
      if first < second {
        machine.instruction_pointer = immediate;
      } else {
        machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      }
    },
    bytes_per_arg: 2,
    clock_cycles: 4,
  },
  Instruction {
    inst: "JMPIE",
    num_args: 1,
    op_code: 0x43,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.stack.push(second);
      machine.stack.push(first);
      if first == second {
        machine.instruction_pointer = immediate;
      } else {
        machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      }
    },
    bytes_per_arg: 2,
    clock_cycles: 4,
  },
  Instruction {
    inst: "JMPIT",
    num_args: 1,
    op_code: 0x44,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      if machine.flags.test {
        machine.instruction_pointer = immediate;
      } else {
        machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      }
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "JMP",
    num_args: 0,
    op_code: 0x45,
    run: &|machine: &mut Machine| {
      let address: u16 = machine.stack.pop();

      machine.instruction_pointer = address;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "JMPGT",
    num_args: 0,
    op_code: 0x46,
    run: &|machine: &mut Machine| {
      let address: u16 = machine.stack.pop();
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.stack.push(second);
      machine.stack.push(first);

      if first > second {
        machine.instruction_pointer = address;
      } else {
        machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      }
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "JMPLT",
    num_args: 0,
    op_code: 0x47,
    run: &|machine: &mut Machine| {
      let address: u16 = machine.stack.pop();
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.stack.push(second);
      machine.stack.push(first);

      if first < second {
        machine.instruction_pointer = address;
      } else {
        machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      }
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "JMPEQ",
    num_args: 0,
    op_code: 0x48,
    run: &|machine: &mut Machine| {
      let address: u16 = machine.stack.pop();
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.stack.push(second);
      machine.stack.push(first);

      if first == second {
        machine.instruction_pointer = address;
      } else {
        machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      }
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "JMPT",
    num_args: 0,
    op_code: 0x49,
    run: &|machine: &mut Machine| {
      let address: u16 = machine.stack.pop();

      if machine.flags.test {
        machine.instruction_pointer = address;
      } else {
        machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      }
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "PUSHP",
    num_args: 0,
    op_code: 0x4a,
    run: &|machine: &mut Machine| {
      machine
        .instruction_pointer_stack
        .push(machine.instruction_pointer);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "POPP",
    num_args: 0,
    op_code: 0x4b,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = machine.instruction_pointer_stack.pop();
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "CALL",
    num_args: 0,
    op_code: 0x4c,
    run: &|machine: &mut Machine| {
      let address: u16 = machine.stack.pop();

      machine
        .instruction_pointer_stack
        .push(machine.instruction_pointer);
      machine.instruction_pointer = address;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "RET",
    num_args: 0,
    op_code: 0x4d,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = machine.instruction_pointer_stack.pop();
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "CALLI",
    num_args: 1,
    op_code: 0x4e,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      machine
        .instruction_pointer_stack
        .push(machine.instruction_pointer);
      machine.instruction_pointer = immediate;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "SKIP",
    num_args: 1,
    op_code: 0x4f,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 2;
    },
    bytes_per_arg: 2,
    clock_cycles: 1,
  },
  Instruction {
    inst: "LOAD",
    num_args: 0,
    op_code: 0x50,
    run: &|machine: &mut Machine| {
      let address: u16 = machine.stack.pop();
      let part1: u16 = machine.memory[address as usize] as u16;
      let address2: u16 = ((address as usize + 1) % 65536) as u16;
      let part2: u16 = (machine.memory[address2 as usize] as u16) << 8;
      let loaded_data = part1 | part2;
      machine.stack.push(loaded_data);

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 3,
  },
  Instruction {
    inst: "STORE",
    num_args: 0,
    op_code: 0x51,
    run: &|machine: &mut Machine| {
      let address: u16 = machine.stack.pop();
      let data: u16 = machine.stack.pop();
      let part1: u8 = (data & 0xff) as u8;
      let part2: u8 = ((data >> 8) & 0xff) as u8;
      machine.memory[address as usize] = part1;
      let address2: u16 = ((address as usize + 1) % 65536) as u16;
      machine.memory[address2 as usize] = part2;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 3,
  },
  Instruction {
    inst: "LOADI",
    num_args: 1,
    op_code: 0x52,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let apart1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let apart2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let address = apart1 | apart2;
      let part1: u16 = machine.memory[address as usize] as u16;
      let address2: u16 = ((address as usize + 1) % 65536) as u16;
      let part2: u16 = (machine.memory[address2 as usize] as u16) << 8;
      let loaded_data = part1 | part2;
      machine.stack.push(loaded_data);

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 5,
  },
  Instruction {
    inst: "STOREI",
    num_args: 1,
    op_code: 0x53,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let apart1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let apart2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let address = apart1 | apart2;
      let data: u16 = machine.stack.pop();
      let part1: u8 = (data & 0xff) as u8;
      let part2: u8 = ((data >> 8) & 0xff) as u8;
      machine.memory[address as usize] = part1;
      let address2: u16 = ((address as usize + 1) % 65536) as u16;
      machine.memory[address2 as usize] = part2;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 5,
  },
  Instruction {
    inst: "OR",
    num_args: 0,
    op_code: 0x60,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.accumulator = first | second;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "AND",
    num_args: 0,
    op_code: 0x61,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.accumulator = first & second;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "XOR",
    num_args: 0,
    op_code: 0x62,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.accumulator = first ^ second;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "NAND",
    num_args: 0,
    op_code: 0x63,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.accumulator = (first & second) ^ 0xffff;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "NOR",
    num_args: 0,
    op_code: 0x64,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.accumulator = (first | second) ^ 0xffff;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "NOT",
    num_args: 0,
    op_code: 0x65,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.accumulator = first ^ 0xffff;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "LSFT0",
    num_args: 0,
    op_code: 0x66,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.accumulator = first << 1;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "RSFT0",
    num_args: 0,
    op_code: 0x67,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.accumulator = first >> 1;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "LSFT1",
    num_args: 0,
    op_code: 0x68,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.accumulator = (first << 1) | 0x1;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "RSFT1",
    num_args: 0,
    op_code: 0x69,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.accumulator = (first >> 1) | 0x1 << 15;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "ORI",
    num_args: 1,
    op_code: 0x6a,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      let popped: u16 = machine.stack.pop();

      machine.accumulator = popped | immediate;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "ANDI",
    num_args: 1,
    op_code: 0x6b,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      let popped: u16 = machine.stack.pop();

      machine.accumulator = popped & immediate;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "XORI",
    num_args: 1,
    op_code: 0x6c,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      let popped: u16 = machine.stack.pop();

      machine.accumulator = popped ^ immediate;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "NANDI",
    num_args: 1,
    op_code: 0x6d,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      let popped: u16 = machine.stack.pop();

      machine.accumulator = (popped & immediate) ^ 0xffff;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "NORI",
    num_args: 1,
    op_code: 0x6e,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      let popped: u16 = machine.stack.pop();

      machine.accumulator = (popped | immediate) ^ 0xffff;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "INC",
    num_args: 0,
    op_code: 0x70,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.accumulator = (popped as u32 + 1) as u16;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "INC2",
    num_args: 0,
    op_code: 0x71,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.accumulator = (popped as u32 + 2) as u16;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "INC3",
    num_args: 0,
    op_code: 0x72,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.accumulator = (popped as u32 + 3) as u16;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "INC4",
    num_args: 0,
    op_code: 0x73,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.accumulator = (popped as u32 + 4) as u16;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "DEC",
    num_args: 0,
    op_code: 0x74,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.accumulator = (((popped as i32 - 1) + 65536) % 65536) as u16;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "DEC2",
    num_args: 0,
    op_code: 0x75,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.accumulator = (((popped as i32 - 2) + 65536) % 65536) as u16;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "DEC3",
    num_args: 0,
    op_code: 0x76,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.accumulator = (((popped as i32 - 3) + 65536) % 65536) as u16;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "DEC4",
    num_args: 0,
    op_code: 0x77,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.accumulator = (((popped as i32 - 4) + 65536) % 65536) as u16;

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "INCP",
    num_args: 0,
    op_code: 0x78,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.stack.push((popped as u32 + 1) as u16);

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "INC2P",
    num_args: 0,
    op_code: 0x79,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.stack.push((popped as u32 + 2) as u16);

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "INC3P",
    num_args: 0,
    op_code: 0x7a,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.stack.push((popped as u32 + 3) as u16);

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "INC4P",
    num_args: 0,
    op_code: 0x7b,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine.stack.push((popped as u32 + 4) as u16);

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "DECP",
    num_args: 0,
    op_code: 0x7c,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine
        .stack
        .push((((popped as i32 - 1) + 65536) % 65536) as u16);

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "DEC2P",
    num_args: 0,
    op_code: 0x7d,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine
        .stack
        .push((((popped as i32 - 2) + 65536) % 65536) as u16);

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "DEC3P",
    num_args: 0,
    op_code: 0x7e,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine
        .stack
        .push((((popped as i32 - 3) + 65536) % 65536) as u16);

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "DEC4P",
    num_args: 0,
    op_code: 0x7f,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();

      machine
        .stack
        .push((((popped as i32 - 4) + 65536) % 65536) as u16);

      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "TEST",
    num_args: 0,
    op_code: 0x80,
    run: &|machine: &mut Machine| {
      machine.flags.test = machine.flags.carry || machine.flags.overflow;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "TESTC",
    num_args: 0,
    op_code: 0x81,
    run: &|machine: &mut Machine| {
      machine.flags.test = machine.flags.carry;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "TESTO",
    num_args: 0,
    op_code: 0x82,
    run: &|machine: &mut Machine| {
      machine.flags.test = machine.flags.overflow;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "TSAST",
    num_args: 0,
    op_code: 0x88,
    run: &|machine: &mut Machine| {
      machine.flags.test = machine.stack.pop() != 0;
      machine.stack.push(1);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 3,
  },
  Instruction {
    inst: "TSAINC",
    num_args: 0,
    op_code: 0x89,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();
      machine.flags.test = popped != 0;
      machine.stack.push((popped as u32 + 1) as u16);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 4,
  },
  Instruction {
    inst: "TSADEC",
    num_args: 0,
    op_code: 0x8a,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();
      machine.flags.test = popped != 0;
      machine
        .stack
        .push((((popped as i32 - 1) + 65536) % 65536) as u16);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 4,
  },
  Instruction {
    inst: "TSASTR",
    num_args: 0,
    op_code: 0x8b,
    run: &|machine: &mut Machine| {
      machine.flags.test = machine.registers[0] != 0;
      machine.registers[0] = 1;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "TSAINR",
    num_args: 0,
    op_code: 0x8c,
    run: &|machine: &mut Machine| {
      machine.flags.test = machine.registers[0] != 0;
      machine.registers[0] = (machine.registers[0] as u32 + 1) as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "TSADER",
    num_args: 0,
    op_code: 0x8d,
    run: &|machine: &mut Machine| {
      machine.flags.test = machine.registers[0] != 0;
      machine.registers[0] = (((machine.registers[0] as i32 - 1) + 65536) % 65536) as u16;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 2,
  },
  Instruction {
    inst: "LSFTB",
    num_args: 0,
    op_code: 0x90,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();
      machine.accumulator = popped << 8;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "RSFTB",
    num_args: 0,
    op_code: 0x91,
    run: &|machine: &mut Machine| {
      let popped: u16 = machine.stack.pop();
      machine.accumulator = popped >> 8;
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "PRN",
    num_args: 0,
    op_code: 0xf0,
    run: &|machine: &mut Machine| {
      let popped = machine.stack.pop();
      machine.output_buffer.put((popped & 0x00ff) as u8);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "PRNI",
    num_args: 1,
    op_code: 0xf1,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let immediate: u8 = machine.memory[machine.instruction_pointer as usize];

      machine.output_buffer.put(immediate);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 1,
    clock_cycles: 2,
  },
  Instruction {
    inst: "PRN2",
    num_args: 0,
    op_code: 0xf2,
    run: &|machine: &mut Machine| {
      let popped = machine.stack.pop();
      machine.output_buffer.put((popped & 0x00ff) as u8);
      machine.output_buffer.put(((popped >> 8) & 0x00ff) as u8);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "PRN2I",
    num_args: 1,
    op_code: 0xf3,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part1: u8 = machine.memory[machine.instruction_pointer as usize];
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
      let part2: u8 = machine.memory[machine.instruction_pointer as usize];

      machine.output_buffer.put(part1);
      machine.output_buffer.put(part2);
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 2,
    clock_cycles: 3,
  },
  Instruction {
    inst: "DUMP8",
    num_args: 0,
    op_code: 0xf4,
    run: &|machine: &mut Machine| {
      let popped = machine.stack.pop();
      machine
        .output_buffer
        .put_string(format!("0x{:02x}", (popped & 0x00ff) as u8));
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
  Instruction {
    inst: "DUMP16",
    num_args: 0,
    op_code: 0xf5,
    run: &|machine: &mut Machine| {
      let popped = machine.stack.pop();
      machine
        .output_buffer
        .put_string(format!("0x{:04x}", popped));
      machine.instruction_pointer = (machine.instruction_pointer as u32 + 1) as u16;
    },
    bytes_per_arg: 0,
    clock_cycles: 1,
  },
];
