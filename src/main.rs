//! Rusty Jello is a simple (and a bit hack-y) language created by Jacob Allen

pub mod arguments;
pub mod machine;
pub mod assembler;
pub mod instructions;

use arguments::Args;
use machine::Machine;
use assembler::Assembler;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::time::Instant;

fn main() {
  println!("Rusty Jello v0.7.0 (Rumbustious) by Jacob Allen");

  let args: Args = Args::new(
    env::args().collect(),
    vec!["-v".to_string(), "-h".to_string()],
  );
  let mut input_file: String = "".to_string();
  let debug_level: u8;
  let tick_rate: f64;

  if args.has_arg("") {
    if args.count_arg("") > 1 {
      println!("Rusty Jello only needs 1 input file name")
    } else {
      match args.get_arg("") {
        Some(arg) => input_file = arg.value.to_string(),
        None => input_file = "".to_string(),
      }
    }
  }

  if args.has_arg("-h") {
    println!("Usage: rusty_jello FILE_NAME [flags] [options]");
    println!("Flags:");
    println!("  -v: Displays version");
    println!("  -h: Displays this help section");
    println!("Options:");
    println!("  -dbl: Sets the debug level, can be 0 to 2");
    println!("  -t: Sets internal clock rate in hertz");
    return;
  }

  if args.has_arg("-v") {
    return;
  }

  match args.get_arg("-dbl") {
    Some(arg) => {
      let value: String = arg.value.to_string();
      match value.parse::<u8>() {
        Ok(val) => debug_level = val,
        Err(err) => {
          println!("Invalid debug level specified, {}", err);
          return;
        }
      }
    }
    None => debug_level = 0,
  }

  match args.get_arg("-t") {
    Some(arg) => {
      let value: String = arg.value.to_string();
      match value.parse::<f64>() {
        Ok(val) => tick_rate = val,
        Err(err) => {
          println!("Invalid clock rate specified, {}", err);
          return;
        }
      }
    }
    None => tick_rate = 0.0,
  }

  if input_file == "" {
    println!("Rusty Jello requires an input file to run");
    return;
  }

  print!("Reading file... ");

  let mut file: File;
  let input_file_path: &Path = Path::new(&input_file);
  let mut code: String = String::new();

  match File::open(input_file_path) {
    Ok(_file) => file = _file,
    Err(err) => {
      match err.kind() {
        ErrorKind::NotFound => println!("Input file '{}' does not exist", input_file),
        ErrorKind::PermissionDenied => println!("Input file access denied"),
        _ => println!("Error opening file, {:?}", err),
      }
      return;
    }
  }

  match file.read_to_string(&mut code) {
    Ok(..) => println!("Done."),
    Err(err) => {
      println!("Error reading file, {:?}", err);
      return;
    }
  }

  let mut assembler: Assembler = Assembler::new();
  let bytecode: Vec<u8>;

  print!("Assembling file... ");

  assembler.add_string(code.as_str());

  let assembly_start_time = Instant::now();
  match assembler.assemble() {
    Ok(generated_bytecode) => {
      bytecode = generated_bytecode;
    }
    Err(err) => {
      println!("Failed!");
      println!("{}", err);
      return;
    }
  }

  let assembly_elapsed: f64 = assembly_start_time.elapsed().as_secs() as f64
    + (assembly_start_time.elapsed().subsec_nanos() as f64 / 10_000_000f64);

  println!("Done.");

  if debug_level > 0 {
    println!("Assembly took {} seconds", assembly_elapsed);
  }

  print!("Loading bytecode into virtual machine... ");

  let mut machine: Machine = Machine::new();
  machine.clock_speed_hz = tick_rate;

  let mut pointer: usize = 0;
  for byte in bytecode {
    machine.memory[pointer] = byte;
    pointer += 1;
  }

  println!("Done.");

  println!("Running virtual machine until halt... ");

  println!("Initial {:?}", machine);
  let execution_start_time = Instant::now();
  while !machine.flags.halt {
    if debug_level > 1 {
      println!("{:?}", machine);
    }
    if debug_level > 0 {
      println!("{}", machine.format_inst());
    }
    machine.step();
  }
  let execution_elapsed: f64 = execution_start_time.elapsed().as_secs() as f64
    + (execution_start_time.elapsed().subsec_nanos() as f64 / 10_000_000f64);
  println!("Final {:?}", machine);

  if debug_level > 0 {
    println!("Execution took {} seconds", execution_elapsed);
  }

  println!("Done.");
}
