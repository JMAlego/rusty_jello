//! Module for parsing args simply

use std::fmt;

/// Struct representing arguments as name & value pairs
pub struct Arg {
  pub name: String,
  pub value: String,
}

impl fmt::Debug for Arg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Arg {{ name: {}, value: {} }}", self.name, self.value)
  }
}

/// Struct for representing a list of arguments and performing operations on that list
pub struct Args {
  args: Vec<Arg>,
}

/// Gets an argument by name, if it exists
impl Args {
  pub fn get_arg(&self, name: &str) -> Option<&Arg> {
    for arg in &self.args {
      if name.to_string() == arg.name {
        return Some(&arg);
      }
    }
    return None;
  }
}

/// Checks if argument exists by name
impl Args {
  pub fn has_arg(&self, name: &str) -> bool {
    for arg in &self.args {
      if name.to_string() == arg.name {
        return true;
      }
    }
    return false;
  }
}

/// Counts occurrences of an argument by name
impl Args {
  pub fn count_arg(&self, name: &str) -> usize {
    let mut count: usize = 0;
    for arg in &self.args {
      if name.to_string() == arg.name {
        count += 1;
      }
    }
    return count;
  }
}

///Generates a new Args struct
impl Args {
  pub fn new(args: Vec<String>, flags: Vec<String>) -> Args {
    let mut results: Vec<Arg> = Vec::new();
    let mut pos: usize = 1;
    if args.len() > 0 {
      while pos < args.len() {
        let item: &String = &args[pos];
        pos += 1;
        if item.starts_with('-') {
          if flags.contains(item) {
            results.push(Arg {
              name: item.to_string(),
              value: "true".to_string(),
            })
          } else {
            if pos < args.len() {
              let val: &String = &args[pos];
              results.push(Arg {
                name: item.to_string(),
                value: val.to_string(),
              });
              pos += 1;
            } else {
              results.push(Arg {
                name: item.to_string(),
                value: "".to_string(),
              });
            }
          }
        } else {
          results.push(Arg {
            name: "".to_string(),
            value: item.to_string(),
          });
        }
      }
    }
    return Args { args: results };
  }
}

impl fmt::Debug for Args {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Args {:?}", self.args)
  }
}
