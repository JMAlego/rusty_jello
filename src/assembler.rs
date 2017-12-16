//! Module for compiling lines of code into byte code for the Rusty Jello machine

use instructions;
use std::error::Error;
use std::fmt;
use instructions::Instruction;

#[derive(Clone)]
enum ByteType {
  Data,
  Label,
  LabelPadding,
  OpCode,
}

impl fmt::Display for ByteType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &ByteType::Data => write!(f, "Data"),
      &ByteType::Label => write!(f, "Label"),
      &ByteType::LabelPadding => write!(f, "LabelPadding"),
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
pub struct AssembleError {
  error_type: String,
  error_description: String,
}

impl fmt::Display for AssembleError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.error_type, self.error_description)
  }
}

impl Error for AssembleError {
  fn description(&self) -> &str {
    "Compiler Error"
  }
}

pub struct Assembler {
  program: String,
}

impl Assembler {
  pub fn new() -> Assembler {
    return Assembler {
      program: "".to_string(),
    };
  }
}

impl Assembler {
  pub fn add_string(&mut self, new_lines: &str) {
    self.program.push_str(new_lines);
  }
}

impl Assembler {
  pub fn assemble(&mut self) -> Result<Vec<u8>, String> {
    let mut result: Vec<u8> = Vec::new();
    self.check_labels()?;
    let partially_compiled_bytecode = self.assemble_program_to_bytecode()?;
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
            address += 1;
          }
        }
        ByteType::OpCode => result.push(byte.byte_value),
        ByteType::LabelPadding => {}
      }
    }

    return Ok(result);
  }
}

impl Assembler {
  fn assemble_program_to_bytecode(&mut self) -> Result<Vec<Byte>, String> {
    let mut result: Vec<Byte> = Vec::new();
    let mut line_counter: usize = 1;
    let mut labels_from_previous: Vec<String> = Vec::new();
    let mut data_entries: Vec<(u16, Vec<Byte>)> = Vec::new();
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
      if line.trim().starts_with(".DATA") {
        let split_line = Assembler::split_line(line);
        if split_line.len() == 3 {
          if let Some(_address) = Assembler::parse_value(split_line[1].as_str()) {
            let address: u16;
            if _address.len() == 2 {
              address = (_address[0].byte_value as u16) | ((_address[1].byte_value as u16) << 8);
            } else {
              return Err(format!("Data Address Length Error on line {}", line_counter).to_string());
            }
            if let Some(data) = Assembler::parse_value(split_line[2].as_str()) {
              data_entries.push((address, data));
            } else {
              return Err(format!("Data Value Error on line {}", line_counter).to_string());
            }
          } else {
            return Err(format!("Data Address Error on line {}", line_counter).to_string());
          }
        } else {
          return Err(format!("Data Length Error on line {}", line_counter).to_string());
        }
        line_counter += 1;
        continue;
      }
      match self.assemble_line_to_bytecode(line) {
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
    for entry in data_entries {
      let (address, data) = entry;
      if result.len() >= address as usize {
        return Err(format!("Data attempted to overwrite address at {:04x}", address).to_string());
      }
      while result.len() < address as usize {
        result.push(Byte::from_u8(0x00));
      }
      for data_byte in data {
        result.push(data_byte);
      }
    }
    return Ok(result);
  }
}

impl Assembler {
  pub fn check_labels(&mut self) -> Result<(), String> {
    let mut line_counter: usize = 1;
    let mut already_used: Vec<String> = Vec::new();
    for line in self.program.trim().lines() {
      if line.trim().starts_with(':') {
        let label: String = line.trim().chars().skip(1).collect::<String>();
        if already_used.contains(&label) {
          return Err(format!(
            "Ambiguous Input Error: redefinition of label \"{}\" on line {}",
            label,
            line_counter
          ));
        }
        already_used.push(label);
      }
      line_counter += 1;
    }
    return Ok(());
  }
}

impl Assembler {
  fn split_line(line: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut line_item: String = String::new();
    let mut in_line_item: bool = false;
    let mut in_quote = false;
    let mut escape = false;
    for item in line.trim().chars() {
      if item == ' ' {
        if in_line_item {
          if in_quote {
            line_item.push(item);
          } else {
            in_line_item = false;
            result.push(line_item.clone());
            line_item = "".to_string();
          }
        }
        escape = false;
      } else {
        if !in_line_item {
          in_line_item = true;
        }
        if item == '\\' {
          escape = true;
          line_item.push(item);
          continue;
        }
        if item == '"' || item == '\'' {
          if !escape {
            if in_quote {
              in_quote = false;
            } else {
              in_quote = true;
            }
          }
        }
        line_item.push(item);
        escape = false;
      }
    }
    if in_line_item {
      result.push(line_item.clone());
    }
    return result;
  }
}

impl Assembler {
  fn assemble_line_to_bytecode(&self, line: &str) -> Result<Vec<Byte>, AssembleError> {
    let mut result: Vec<Byte> = Vec::new();
    let split_line: Vec<String> = Assembler::split_line(line);
    let inst: Instruction;

    if split_line.len() > 0 {
      match instructions::find_inst_by_name(split_line[0].as_str()) {
        Some(i) => {
          inst = i;
        }
        None => {
          return Err(AssembleError {
            error_type: "Syntax Error".to_string(),
            error_description: format!("unknown instruction \"{}\"", split_line[0]),
          })
        }
      }
      if inst.num_args as usize != split_line.len() - 1 {
        return Err(AssembleError {
          error_type: "Syntax Error".to_string(),
          error_description: format!(
            "argument count mismatch, expected {} but got {},",
            inst.num_args,
            split_line.len() - 1
          ),
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
        let rest = split_line.clone().split_off(1);

        for item in rest {
          match Assembler::parse_value(item.as_str()) {
            Some(bytes) => for byte in bytes {
              result.push(byte);
              byte_sum += 1;
            },
            None => {
              return Err(AssembleError {
                error_type: "Value Error".to_string(),
                error_description: format!("could not parse argument \"{}\"", item),
              })
            }
          }
        }
      }

      let inst_size: usize = (inst.bytes_per_arg as usize * inst.num_args as usize) as usize;

      if byte_sum > inst_size {
        return Err(AssembleError {
          error_type: "Syntax Error".to_string(),
          error_description: format!(
            "argument(s) too many bytes, expected {} but got {},",
            inst_size,
            byte_sum
          ),
        });
      }
      if byte_sum < inst_size {
        return Err(AssembleError {
          error_type: "Syntax Error".to_string(),
          error_description: format!(
            "argument(s) too few bytes, expected {} but got {},",
            inst_size,
            byte_sum
          ),
        });
      }
    }

    return Ok(result);
  }
}

impl Assembler {
  fn parse_value(to_parse: &str) -> Option<Vec<Byte>> {
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
          }
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
      });
      result.push(Byte {
        byte_type: ByteType::LabelPadding,
        byte_value: 0,
        byte_label: to_compare.chars().skip(1).collect::<String>(),
        byte_attached_labels: vec![],
      });
    }

    if to_compare.starts_with("'") && to_compare.ends_with("'") && to_compare.len() == 3 {
      result.push(Byte::from_u8(to_compare.as_bytes()[1] as u8));
    }

    if to_compare.starts_with("\"") && to_compare.ends_with("\"") && to_compare.len() > 2 {
      let mut escape = false;
      let mut complete = false;
      for chr in to_compare.chars().skip(1) {
        if chr == '"' && !escape {
          complete = true;
          break;
        } else if chr == '0' && escape {
          escape = false;
          result.push(Byte::from_u8(0));
        } else if chr == 'n' && escape {
          escape = false;
          result.push(Byte::from_u8('\n' as u8));
        } else if chr == 'r' && escape {
          escape = false;
          result.push(Byte::from_u8('\r' as u8));
        } else if chr == 't' && escape {
          escape = false;
          result.push(Byte::from_u8('\t' as u8));
        } else if chr == '\\' {
          if escape {
            result.push(Byte::from_u8(chr as u8));
            escape = false;
          } else {
            escape = true;
          }
        } else {
          escape = false;
          result.push(Byte::from_u8(chr as u8));
        }
      }
      if !complete {
        return None;
      }
    }

    if result.len() == 0 {
      return None;
    }

    return Some(result);
  }
}
