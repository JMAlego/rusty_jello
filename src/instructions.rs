//! Module for storing instruction definitions
use machine::Machine;

pub struct Instruction<'a> {
  inst: &'a str,
  num_args: u8,
  op_code: u8,
  run: &'a Fn(&mut Machine),
}

pub const INSTRUCTION_COUNT: usize = 8;

pub const INSTRUCTIONS: [Instruction; INSTRUCTION_COUNT] = [
  Instruction {
    inst: "NOOP",
    num_args: 0,
    op_code: 0x0,
    run: &|machine: &mut Machine| {
      machine.ip_inc();
    }
  },
  Instruction {
    inst: "HALT",
    num_args: 0,
    op_code: 0xff,
    run: &|machine: &mut Machine| {
      machine.flag_set_halt(true);
    }
  },
  Instruction {
    inst: "PUSH",
    num_args: 0,
    op_code: 0x10,
    run: &|x: &mut Machine| {}
  },
  Instruction {
    inst: "POP",
    num_args: 0,
    op_code: 0x11,
    run: &|x: &mut Machine| {}
  },
  Instruction {
    inst: "ADD",
    num_args: 0,
    op_code: 0x20,
    run: &|x: &mut Machine| {}
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
