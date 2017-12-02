//! Representation of the Rusty Jello machine

pub enum Register{
  R0 = 0,
  R1 = 1,
  R2 = 2,
  R3 = 3
}

pub struct Flags {
  pub halt: bool,
  pub carry: bool,
  pub overflow: bool,
  pub test: bool,
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
      self.stack_pointer += 1;
      self.stack[self.stack_pointer as usize] = item;
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
      return self.stack[self.stack_pointer as usize + 1];
    }
    return 0;
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
    };
  }
}
