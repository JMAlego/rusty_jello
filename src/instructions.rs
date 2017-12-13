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
    if INSTRUCTIONS[index].inst == name.to_uppercase() {
      return Some(INSTRUCTIONS[index].clone());
    }
  }
  return None;
}

pub const INSTRUCTION_COUNT: usize = 51;

pub const INSTRUCTIONS: [Instruction; INSTRUCTION_COUNT] = [
  Instruction {
    inst: "NOOP",
    num_args: 0,
    op_code: 0x0,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "HALT",
    num_args: 0,
    op_code: 0x08,
    run: &|machine: &mut Machine| {
      machine.flags.halt = true;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "LRI",
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
    bytes_per_arg: 2,
  },
  Instruction {
    inst: "LR0",
    num_args: 0,
    op_code: 0x11,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[0];
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "LR1",
    num_args: 0,
    op_code: 0x12,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[1];
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "LR2",
    num_args: 0,
    op_code: 0x13,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[2];
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "LR3",
    num_args: 0,
    op_code: 0x14,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.registers[3];
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "SR0",
    num_args: 0,
    op_code: 0x15,
    run: &|machine: &mut Machine| {
      machine.registers[0] = machine.accumulator;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "SR1",
    num_args: 0,
    op_code: 0x16,
    run: &|machine: &mut Machine| {
      machine.registers[1] = machine.accumulator;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "SR2",
    num_args: 0,
    op_code: 0x17,
    run: &|machine: &mut Machine| {
      machine.registers[2] = machine.accumulator;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "SR3",
    num_args: 0,
    op_code: 0x18,
    run: &|machine: &mut Machine| {
      machine.registers[3] = machine.accumulator;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "ZERO",
    num_args: 0,
    op_code: 0x19,
    run: &|machine: &mut Machine| {
      machine.accumulator = 0;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "LRS",
    num_args: 1,
    op_code: 0x1a,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
      machine.accumulator = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 1,
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
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
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
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
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
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
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
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "NEG",
    num_args: 0,
    op_code: 0x24,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      machine.accumulator = input1 ^ 65535;
      machine.accumulator = (machine.accumulator as u32 + 1u32) as u16;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
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
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
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
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "ADDI",
    num_args: 1,
    op_code: 0x27,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      machine.instruction_pointer += 1;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
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
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 2,
  },
  Instruction {
    inst: "SUBI",
    num_args: 1,
    op_code: 0x28,
    run: &|machine: &mut Machine| {
      let input1: u16 = machine.stack.pop();
      machine.instruction_pointer += 1;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
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
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 2,
  },
  Instruction {
    inst: "PUSH",
    num_args: 0,
    op_code: 0x30,
    run: &|machine: &mut Machine| {
      machine.stack.push(machine.accumulator);
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "POP",
    num_args: 0,
    op_code: 0x31,
    run: &|machine: &mut Machine| {
      machine.accumulator = machine.stack.pop();
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
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
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "PEAK",
    num_args: 0,
    op_code: 0x33,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.stack.push(first);
      machine.accumulator = first;
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "SPILL",
    num_args: 0,
    op_code: 0x34,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.stack.pop();
      machine.stack.push(first);
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "DROP",
    num_args: 0,
    op_code: 0x35,
    run: &|machine: &mut Machine| {
      machine.stack.pop();
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "UNDER",
    num_args: 0,
    op_code: 0x36,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.stack.push(machine.accumulator);
      machine.stack.push(first);
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
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
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
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
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "DUP",
    num_args: 0,
    op_code: 0x39,
    run: &|machine: &mut Machine| {
      let first: u16 = machine.stack.pop();
      machine.stack.push(first);
      machine.stack.push(first);
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "PUSHI",
    num_args: 1,
    op_code: 0x3a,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;
      machine.stack.push(immediate);
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 2,
  },
  Instruction {
    inst: "JMPI",
    num_args: 1,
    op_code: 0x40,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;
      machine.instruction_pointer = immediate;
    },
    bytes_per_arg: 2,
  },
  Instruction {
    inst: "JMPIG",
    num_args: 1,
    op_code: 0x41,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;
      
      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.stack.push(second);
      machine.stack.push(first);
      if first > second {
        machine.instruction_pointer = immediate;
      }else{
        machine.instruction_pointer += 1;
      }
    },
    bytes_per_arg: 2,
  },
  Instruction {
    inst: "JMPIL",
    num_args: 1,
    op_code: 0x42,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.stack.push(second);
      machine.stack.push(first);
      if first < second {
        machine.instruction_pointer = immediate;
      }else{
        machine.instruction_pointer += 1;
      }
    },
    bytes_per_arg: 2,
  },
  Instruction {
    inst: "JMPIE",
    num_args: 1,
    op_code: 0x43,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      let first: u16 = machine.stack.pop();
      let second: u16 = machine.stack.pop();
      machine.stack.push(second);
      machine.stack.push(first);
      if first == second {
        machine.instruction_pointer = immediate;
      }else{
        machine.instruction_pointer += 1;
      }
    },
    bytes_per_arg: 2,
  },
  Instruction {
    inst: "JMPIT",
    num_args: 1,
    op_code: 0x44,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      if machine.flags.test {
        machine.instruction_pointer = immediate;
      }else{
        machine.instruction_pointer += 1;
      }
    },
    bytes_per_arg: 2,
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
      }else{
        machine.instruction_pointer += 1;
      }
    },
    bytes_per_arg: 0,
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
      }else{
        machine.instruction_pointer += 1;
      }
    },
    bytes_per_arg: 0,
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
      }else{
        machine.instruction_pointer += 1;
      }
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "JMPT",
    num_args: 0,
    op_code: 0x49,
    run: &|machine: &mut Machine| {
      let address: u16 = machine.stack.pop();

      if machine.flags.test {
        machine.instruction_pointer = address;
      }else{
        machine.instruction_pointer += 1;
      }
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "PUSHI",
    num_args: 0,
    op_code: 0x4a,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer_stack.push(machine.instruction_pointer);
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "POPI",
    num_args: 0,
    op_code: 0x4b,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = machine.instruction_pointer_stack.pop();
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "CALL",
    num_args: 0,
    op_code: 0x4c,
    run: &|machine: &mut Machine| {
      let address: u16 = machine.stack.pop();

      machine.instruction_pointer_stack.push(machine.instruction_pointer);
      machine.instruction_pointer = address;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "RET",
    num_args: 0,
    op_code: 0x4d,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer = machine.instruction_pointer_stack.pop();
      machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "CALLI",
    num_args: 1,
    op_code: 0x4e,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 1;
      let part1: u16 = machine.memory[machine.instruction_pointer as usize] as u16;
      machine.instruction_pointer += 1;
      let part2: u16 = (machine.memory[machine.instruction_pointer as usize] as u16) << 8;
      let immediate = part1 | part2;

      machine.instruction_pointer_stack.push(machine.instruction_pointer);
      machine.instruction_pointer = immediate;
    },
    bytes_per_arg: 2,
  },
  Instruction {
    inst: "SKIP",
    num_args: 1,
    op_code: 0x4f,
    run: &|machine: &mut Machine| {
      machine.instruction_pointer += 2;
    },
    bytes_per_arg: 2,
  },
  Instruction {
    inst: "TESTC",
    num_args: 0,
    op_code: 0x81,
    run: &|machine: &mut Machine| {
     machine.flags.test = machine.flags.carry;
     machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
  Instruction {
    inst: "TESTO",
    num_args: 0,
    op_code: 0x82,
    run: &|machine: &mut Machine| {
     machine.flags.test = machine.flags.overflow;
     machine.instruction_pointer += 1;
    },
    bytes_per_arg: 0,
  },
];
