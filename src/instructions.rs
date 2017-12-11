//! Module for storing instruction definitions
use machine::Machine;

#[derive(Clone)]
pub struct Instruction<'x> {
  pub inst: &'x str,
  pub num_args: u8,
  pub op_code: u8,
  pub run: &'x Fn(&mut Machine),
  pub bytes_per_arg: u8,
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
    if INSTRUCTIONS[index].inst == name {
      return Some(INSTRUCTIONS[index].clone());
    }
  }
  return None;
}

pub const INSTRUCTION_COUNT: usize = 19;

pub const INSTRUCTIONS: [Instruction; INSTRUCTION_COUNT] = [
  Instruction {
    inst: "NOOP",
    num_args: 0,
    op_code: 0x0,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "HALT",
    num_args: 0,
    op_code: 0x08,
    run: &|machine: &mut Machine| {
      machine.flags.halt = true;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "LOADI",
    num_args: 1,
    op_code: 0x10,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      machine.accumulator = part1 | part2;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 2
  },
  Instruction {
    inst: "LOADA",
    num_args: 0,
    op_code: 0x11,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[0];
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "LOADB",
    num_args: 0,
    op_code: 0x12,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[1];
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "LOADC",
    num_args: 0,
    op_code: 0x13,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[2];
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "LOADD",
    num_args: 0,
    op_code: 0x14,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[3];
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "STORA",
    num_args: 0,
    op_code: 0x15,
    run: &|machine: &mut Machine| {
      machine.registers[0] = machine.accumulator;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "STORB",
    num_args: 0,
    op_code: 0x16,
    run: &|machine: &mut Machine| {
      machine.registers[1] = machine.accumulator;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "STORC",
    num_args: 0,
    op_code: 0x17,
    run: &|machine: &mut Machine| {
      machine.registers[2] = machine.accumulator;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "STORD",
    num_args: 0,
    op_code: 0x18,
    run: &|machine: &mut Machine| {
      machine.registers[3] = machine.accumulator;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "ZERO",
    num_args: 0,
    op_code: 0x19,
    run: &|machine: &mut Machine| {
      machine.accumulator = 0;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "LOADS",
    num_args: 1,
    op_code: 0x1a,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
      machine.accumulator = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 1
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
      }else{
        carry = false;
      }
      machine.accumulator = result as u16;
      machine.flags.carry = carry;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "ADDC",
    num_args: 0,
    op_code: 0x21,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      let input2: u16 = machine.stack.pop();
      let mut result: u32 = input1 as u32 + input2 as u32 + if machine.flags.carry {1} else {0};
      let carry: bool;
      if result > 65535 { 
        carry = true;
        result = result % 65536;
      }else{
        carry = false;
      }
      machine.accumulator = result as u16;
      machine.flags.carry = carry;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
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
      }else{
        carry = false;
      }
      machine.accumulator = result as u16;
      machine.flags.carry = carry;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "SUBC",
    num_args: 0,
    op_code: 0x23,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      let input2: u16 = machine.stack.pop();
      let mut result: i32 = input1 as i32 - input2 as i32 + if machine.flags.carry {-1} else {0};
      let carry: bool;
      if result < 0 {
        carry = true;
        result = 65536 + result;
      }else{
        carry = false;
      }
      machine.accumulator = result as u16;
      machine.flags.carry = carry;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "PUSH",
    num_args: 0,
    op_code: 0x30,
    run: &|machine: &mut Machine| {
      machine.stack.push(machine.accumulator);
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
  Instruction {
    inst: "POP",
    num_args: 0,
    op_code: 0x31,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.stack.pop();
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0
  },
];
