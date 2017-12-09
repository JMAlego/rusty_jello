//! Module for compiling lines of code into byte code for the Rusty Jello machine

use instructions;
use std::error::Error;
use std::fmt;
use instructions::Instruction;

#[derive(Clone)]
enum ByteType {
  Data,
  Label,
  OpCode,
}

impl fmt::Display for ByteType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &ByteType::Data => write!(f, "Data"),
      &ByteType::Label => write!(f, "Label"),
      &ByteType::OpCode => write!(f, "OpCode"),
    }
  }
}

#[derive(Clone)]
struct Byte {
  byte_type: ByteType,
  byte_value: u8,
  byte_label: String,
  byte_attached_labels: Vec<String>,
}

impl Byte {
  fn from_u8(val: u8) -> Byte {
    return Byte {
      byte_type: ByteType::Data,
      byte_value: val,
      byte_label: "".to_string(),
      byte_attached_labels: vec![],
    };
  }
}

impl fmt::Display for Byte {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}: {}|{} ({:?})",
      self.byte_type,
      self.byte_value,
      self.byte_label,
      self.byte_attached_labels,
    )
  }
}

#[derive(Clone)]
struct Label {
  line: usize,
  name: String,
}

impl fmt::Display for Label {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} -> {}", self.line, self.name)
  }
}

impl fmt::Debug for Label {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} -> {}", self.line, self.name)
  }
}

#[derive(Debug)]
pub struct CompilerError<'a> {
  error_type: &'a str,
  error_description: &'a str,
}

impl<'a> fmt::Display for CompilerError<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.error_type, self.error_description)
  }
}

impl<'a> Error for CompilerError<'a> {
  fn description(&self) -> &str {
    "Compiler Error"
  }
}

pub struct Compiler {
  program: String,
  labels: Vec<Label>,
}

impl Compiler {
  pub fn new() -> Compiler {
    return Compiler {
      program: "".to_string(),
      labels: Vec::new(),
    };
  }
}

impl Compiler {
  pub fn add_string(&mut self, new_lines: &str) {
    self.program.push_str(new_lines);
  }
}

impl Compiler {
  pub fn compile(&mut self) -> Result<Vec<u8>, String> {
    let mut result: Vec<u8> = Vec::new();
    self.create_label_table()?;
    let partially_compiled_bytecode = self.compile_program_to_bytecode()?;
    for byte in partially_compiled_bytecode.iter() {
      match byte.byte_type {
        ByteType::Data => result.push(byte.byte_value),
        ByteType::Label => {
          let mut address: u16 = 0;
          for possible_label_byte in partially_compiled_bytecode.iter() {
            if possible_label_byte
              .byte_attached_labels
              .contains(&byte.byte_label)
            {
              result.push((address & 0xff) as u8);
              result.push((address >> 8) as u8);
            }
            match byte.byte_type {
              ByteType::Label => address += 1,
              _ => address += 1,
            }
          }
        }
        ByteType::OpCode => result.push(byte.byte_value),
      }
    }

    return Ok(result);
  }
}

impl Compiler {
  fn compile_program_to_bytecode(&mut self) -> Result<Vec<Byte>, String> {
    let mut result: Vec<Byte> = Vec::new();
    let mut line_counter: usize = 1;
    let mut labels_from_previous: Vec<String> = Vec::new();
    for line in self.program.trim().lines() {
      if line.trim().is_empty() || line.trim().starts_with('#') {
        line_counter += 1;
        continue;
      }
      if line.trim().starts_with(':') {
        labels_from_previous.push(line.trim().chars().skip(1).collect::<String>());
        line_counter += 1;
        continue;
      }
      match self.compile_line_to_bytecode(line) {
        Ok(line_bytecode) => {
          for byte in line_bytecode {
            match byte.byte_type {
              ByteType::OpCode => {
                let mut adjusted_byte: Byte = byte.clone();
                adjusted_byte.byte_attached_labels = labels_from_previous.clone();
                result.push(adjusted_byte);
                labels_from_previous.clear();
              }
              _ => result.push(byte),
            }
          }
          line_counter += 1;
        }
        Err(err) => return Err(format!("{} on line {}", err, line_counter).to_string()),
      }
    }
    return Ok(result);
  }
}

impl Compiler {
  pub fn create_label_table(&mut self) -> Result<(), String> {
    let mut line_counter: usize = 1;
    let mut already_used: Vec<String> = Vec::new();
    for line in self.program.trim().lines() {
      if line.trim().starts_with(':') {
        let label: String = line.trim().chars().skip(1).collect::<String>();
        if already_used.contains(&label) {
          return Err(format!(
            "Preprocessor Error: label redefinition on line {}",
            line_counter
          ));
        }
        self.labels.push(Label {
          name: label.to_string(),
          line: line_counter,
        });
        already_used.push(label);
      }
      line_counter += 1;
    }
    return Ok(());
  }
}

impl Compiler {
  fn compile_line_to_bytecode(&self, line: &str) -> Result<Vec<Byte>, CompilerError> {
    let mut result: Vec<Byte> = Vec::new();
    let mut split_line: Vec<&str> = line.trim().split(' ').collect();
    let inst: Instruction;

    if split_line.len() > 0 {
      match instructions::find_inst(split_line[0]) {
        Some(i) => {
          inst = i;
        }
        None => {
          return Err(CompilerError {
            error_type: "Syntax Error",
            error_description: "unknown instruction",
          })
        }
      }
      if inst.num_args as usize != split_line.len() - 1 {
        return Err(CompilerError {
          error_type: "Syntax Error",
          error_description: "argument count mismatch",
        });
      }

      result.push(Byte {
        byte_type: ByteType::OpCode,
        byte_value: inst.op_code,
        byte_label: "".to_string(),
        byte_attached_labels: vec![],
      });

      let mut byte_sum: usize = 0;
      if split_line.len() > 1 {
        let rest = split_line.split_off(1);

        for item in rest {
          match self.parse_value(item) {
            Some(bytes) => for byte in bytes {
              result.push(byte);
              byte_sum += 1;
            },
            None => {
              return Err(CompilerError {
                error_type: "Value Error",
                error_description: "could not parse argument",
              })
            }
          }
        }
      }

      let inst_size: usize = (inst.bytes_per_arg as usize * inst.num_args as usize) as usize;

      if byte_sum > inst_size {
        return Err(CompilerError {
          error_type: "Syntax Error",
          error_description: "argument(s) too many bytes",
        });
      }
      if byte_sum < inst_size {
        return Err(CompilerError {
          error_type: "Syntax Error",
          error_description: "argument(s) too few bytes",
        });
      }
    }

    return Ok(result);
  }
}

impl Compiler {
  fn parse_value(&self, to_parse: &str) -> Option<Vec<Byte>> {
    let mut result: Vec<Byte> = Vec::new();
    let to_compare = to_parse.trim();

    if to_compare.starts_with("0x") {
      match to_compare.len() - 2 {
        2 => match u8::from_str_radix(
          to_compare
            .chars()
            .skip(2)
            .take(2)
            .collect::<String>()
            .as_str(),
          16,
        ) {
          Ok(val) => result.push(Byte::from_u8(val)),
          Err(..) => return None,
        },
        4 => match u16::from_str_radix(
          to_compare
            .chars()
            .skip(2)
            .take(4)
            .collect::<String>()
            .as_str(),
          16,
        ) {
          Ok(val) => {
            result.push(Byte::from_u8((val & 0xff) as u8));
            result.push(Byte::from_u8((val >> 8) as u8));
          },
          Err(..) => return None,
        },
        _ => return None,
      }
    }

    if to_compare.starts_with("0b") {
      match u8::from_str_radix(
        to_compare
          .chars()
          .skip(2)
          .take(8)
          .collect::<String>()
          .as_str(),
        2,
      ) {
        Ok(val) => result.push(Byte::from_u8(val)),
        Err(..) => return None,
      }
    }

    if to_compare.starts_with(":") {
      result.push(Byte {
        byte_type: ByteType::Label,
        byte_value: 0,
        byte_label: to_compare.chars().skip(1).collect::<String>(),
        byte_attached_labels: vec![],
      })
    }

    if result.len() == 0 {
      return None;
    }

    return Some(result);
  }
}
