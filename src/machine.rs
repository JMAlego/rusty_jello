//! Representation of the Rusty Jello machine

use std::fmt;
use instructions;
use std::time::Duration;
use std::thread;

pub enum Register {
  R0 = 0,
  R1 = 1,
  R2 = 2,
  R3 = 3,
}

pub struct Flags {
  pub halt: bool,
  pub carry: bool,
  pub overflow: bool,
  pub test: bool,
}

impl fmt::Debug for Flags {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{{halt: {}, carry: {}, overflow: {}, test: {}}}",
      self.halt,
      self.carry,
      self.overflow,
      self.test
    )
  }
}

pub struct SerialBuffer {
  buffer: Vec<u8>,
}

impl SerialBuffer {
  pub fn new() -> SerialBuffer {
    return SerialBuffer { buffer: Vec::new() };
  }
}

impl SerialBuffer {
  pub fn put(&mut self, item: u8) -> bool {
    if self.buffer.len() == 256 {
      return false;
    }
    self.buffer.insert(0, item);
    return true;
  }
}

impl SerialBuffer {
  pub fn put_char(&mut self, item: char) -> bool {
    if self.buffer.len() == 256 {
      return false;
    }
    self.buffer.insert(0, item as u8);
    return true;
  }
}

impl SerialBuffer {
  pub fn put_string(&mut self, items: String) -> bool {
    for item in items.chars() {
      if self.buffer.len() == 256 {
        return false;
      }
      self.buffer.insert(0, item as u8);
    }
    return true;
  }
}

impl SerialBuffer {
  pub fn put_all(&mut self, items: Vec<u8>) -> bool {
    for item in items {
      if self.buffer.len() == 256 {
        return false;
      }
      self.buffer.insert(0, item);
    }
    return true;
  }
}

impl SerialBuffer {
  pub fn put_all_char(&mut self, items: Vec<char>) -> bool {
    for item in items {
      if self.buffer.len() == 256 {
        return false;
      }
      self.buffer.insert(0, item as u8);
    }
    return true;
  }
}

impl SerialBuffer {
  pub fn take(&mut self) -> Option<u8> {
    self.buffer.pop()
  }
}

impl SerialBuffer {
  pub fn take_all(&mut self) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    while self.buffer.len() > 0 {
      result.push(self.buffer.pop().unwrap());
    }
    return result;
  }
}

impl SerialBuffer {
  pub fn clear(&mut self) {
    self.buffer.clear()
  }
}

impl SerialBuffer {
  pub fn has_bytes(&self) -> bool {
    self.buffer.len() > 0
  }
}

pub struct Stack {
  stack_pointer: u8,
  stack: [u16; 16],
}

impl Stack {
  pub fn new() -> Stack {
    return Stack {
      stack_pointer: 0,
      stack: [0; 16],
    };
  }
}

impl Stack {
  pub fn push(&mut self, item: u16) -> bool {
    if self.stack_pointer < 15 {
      self.stack[self.stack_pointer as usize] = item;
      self.stack_pointer += 1;
      return true;
    }
    for i in 0..15 {
      self.stack[i] = self.stack[i + 1];
    }
    self.stack[self.stack_pointer as usize] = item;
    return false;
  }
  pub fn pop(&mut self) -> u16 {
    if self.stack_pointer > 0 {
      self.stack_pointer -= 1;
      return self.stack[self.stack_pointer as usize];
    }
    return 0;
  }
}

impl fmt::Debug for Stack {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut stack: String = String::new();
    for index in 0..self.stack_pointer {
      stack = format!("{:04x}", self.stack[index as usize]) + stack.as_str();
      if index + 1 != self.stack_pointer {
        stack = " ".to_string() + stack.as_str();
      }
    }
    write!(f, "{{{}}}", stack)
  }
}

pub struct Machine {
  pub memory: [u8; 65536],
  pub registers: [u16; 4],
  pub accumulator: u16,
  pub instruction_pointer: u16,
  pub stack: Stack,
  pub instruction_pointer_stack: Stack,
  pub flags: Flags,
  pub clock_speed_hz: f64,
  pub output_buffer: SerialBuffer,
  pub input_buffer: SerialBuffer,
}

impl Machine {
  pub fn new() -> Machine {
    return Machine {
      memory: [0; 65536],
      registers: [0; 4],
      accumulator: 0,
      instruction_pointer: 0,
      stack: Stack::new(),
      instruction_pointer_stack: Stack::new(),
      flags: Flags {
        halt: false,
        carry: false,
        overflow: false,
        test: false,
      },
      clock_speed_hz: 0.0,
      output_buffer: SerialBuffer::new(),
      input_buffer: SerialBuffer::new(),
    };
  }
}

impl Machine {
  fn format_memory(&self) -> String {
    let mut result: String = String::new();
    let mut first: bool = true;
    let mut interesting_count: usize = 0;
    for index in 0..65536 {
      if self.memory[index] != 0 {
        if interesting_count == 0 {
          first = true;
        }
        interesting_count = 3;
      } else if interesting_count > 0 {
        interesting_count -= 1;
        if interesting_count == 0 {
          result += format!(" [{:04x}] ... ", index).as_str();
        }
      }
      if interesting_count > 0 {
        if first {
          first = false;
          result += format!("[{:04x}] ", index).as_str();
        } else {
          result += " ";
        }
        result += format!("{:02x}", self.memory[index]).as_str();
      }
    }
    return result;
  }
}

impl Machine {
  pub fn step(&mut self) {
    let loc: u8 = self.memory[self.instruction_pointer as usize];
    if let Some(inst) = instructions::find_inst_by_opcode(&loc) {
      (inst.run)(self);
      if self.clock_speed_hz != 0.0 {
        let instruction_speed: f64 =
          (inst.clock_cycles as f64) * (1.0f64 / self.clock_speed_hz) * 1000.0f64;
        thread::sleep(Duration::from_millis(instruction_speed as u64));
      }
    }
  }
}

impl Machine {
  pub fn format_inst(&mut self) -> String {
    let loc: u8 = self.memory[self.instruction_pointer as usize];
    if let Some(inst) = instructions::find_inst_by_opcode(&loc) {
      let mut data: String = String::new();
      let mut byte_count: u16 = 0;
      for _ in 0..((inst.bytes_per_arg as usize) * (inst.num_args as usize)) {
        if byte_count != 0 {
          data += " ";
          if byte_count % inst.bytes_per_arg as u16 == 0 {
            data += ". ";
          }
        }
        data += format!(
          "{:02x}",
          self.memory[(self.instruction_pointer + 1 + byte_count) as usize]
        ).as_str();
        byte_count += 1;
      }
      return format!(
        "[{:04x}] {} ({:02x} | {})",
        self.instruction_pointer,
        inst.inst,
        loc,
        data
      ).to_string();
    } else {
      return format!("Unknown Instruction {:2x}", loc).to_string();
    }
  }
}

impl fmt::Debug for Machine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Machine {{
          acc: {},
          ip: {},
          ip_stack: {:?},
          stack: {:?},
          flags: {:?},
          memory: {}
        }}",
      format!("{:04x}: ", self.accumulator),
      format!("{:04x}: ", self.instruction_pointer),
      self.instruction_pointer_stack,
      self.stack,
      self.flags,
      self.format_memory(),
    )
  }
}
