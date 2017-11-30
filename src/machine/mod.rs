//! Representation of the Rusty Jello machine

struct Flags{
  halt: bool,
  carry: bool,
  overflow: bool,
  test: bool,
}

struct Stack {
  stack_pointer: u8,
  stack: [u16; 16],
}

impl Stack {
  fn new() -> Stack {
    return Stack {
      stack_pointer: 0,
      stack: [0; 16],
    };
  }
}

impl Stack {
  fn push(&mut self, item: u16) -> bool {
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
}

impl Stack {
  fn pop(&mut self) -> u16 {
    if self.stack_pointer > 0 {
      self.stack_pointer -= 1;
      return self.stack[self.stack_pointer as usize + 1];
    }
    return 0;
  }
}

pub struct Machine {
  memory: [u8; 65536],
  registers: [u16; 4],
  accumulator: u16,
  instruction_pointer: u16,
  stack: Stack,
  instruction_pointer_stack: Stack,
  flags: Flags,
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
      flags: Flags{halt: false, carry: false, overflow: false, test: false}
    };
  }
}

impl Machine {
  pub fn ip_inc(&mut self){
    self.instruction_pointer += 1;
  }
}

impl Machine {
  pub fn flag_set_halt(&mut self, val: bool){
    self.flags.halt = val;
  }
}
