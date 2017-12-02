//! Module for storing instruction definitions
use machine::Machine;

pub struct Instruction<'a> {
  inst: &'a str,
  num_args: u8,
  op_code: u8,
  run: &'a Fn(&mut Machine),
}

pub const INSTRUCTION_COUNT: usize = 9;

pub const INSTRUCTIONS: [Instruction; INSTRUCTION_COUNT] = [
  Instruction {
    inst: "NOOP",
    num_args: 0,
    op_code: 0x0,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
    }
  },
  Instruction {
    inst: "HALT",
    num_args: 0,
    op_code: 0xff,
    run: &|machine: &mut Machine| {
      machine.flags.halt = true;
    }
  },
  Instruction {
    inst: "LOAD",
    num_args: 1,
    op_code: 0x01,
    run: &|machine: &mut Machine| {
      
    }
  },
  Instruction {
    inst: "PUSH",
    num_args: 0,
    op_code: 0x10,
    run: &|machine: &mut Machine| {
      machine.stack.push(machine.accumulator);
    }
  },
  Instruction {
    inst: "POP",
    num_args: 0,
    op_code: 0x11,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.stack.pop();
    }
  },
  Instruction {
    inst: "ADD",
    num_args: 0,
    op_code: 0x20,
    run: &|machine: &mut Machine| {
      let temp1: u16 = machine.stack.pop();
      let temp2: u16 = machine.stack.pop();
      let mut carry: u16 = temp1 & temp2;
      let mut result: u16 = temp1 ^ temp2;
      for _ in 0..16 {
        let shifted_carry: u16 = carry << 1;
        carry = result & shifted_carry;
        result ^= shifted_carry;
      }
      machine.accumulator = result;
    }
  },
  Instruction {
    inst: "ADDC",
    num_args: 0,
    op_code: 0x21,
    run: &|x: &mut Machine| {}
  },
  Instruction {
    inst: "SUB",
    num_args: 0,
    op_code: 0x22,
    run: &|x: &mut Machine| {}
  },
  Instruction {
    inst: "SUBC",
    num_args: 0,
    op_code: 0x23,
    run: &|x: &mut Machine| {}
  },
];
