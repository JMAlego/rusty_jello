//! Rusty Jello is a simple (and a bit hack-y) language created by Jacob Allen

pub mod arguments;
pub mod machine;
pub mod assembler;
pub mod instructions;

use arguments::Args;
use machine::Machine;
use assembler::Assembler;

use std::io;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::time::Instant;
use std::time::Duration;

fn main() {
  let args: Args = Args::new(
    env::args().collect(),
    vec![
      "-v".to_string(),
      "-h".to_string(),
      "-a".to_string(),
      "-m".to_string(),
      "-q".to_string(),
    ],
  );
  let mut input_file: String = "".to_string();
  let mut output_file: String = "a.ja".to_string();
  let debug_level: u8;
  let tick_rate: f64;

  let quiet_mode: bool = args.has_arg("-q");

  if !quiet_mode {
    println!("Rusty Jello v0.7.1 (Rumbustious) by Jacob Allen");
  }

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
    println!("  -a: Assemble only");
    println!("  -m: Measure time");
    println!("  -q: Show only program output");
    println!("  -b: Buffer output");
    println!("Options:");
    println!("  -dbl: Sets the debug level, can be 0 to 2  (default: 0)");
    println!("  -t: Sets internal clock rate in hertz (default: 0)");
    println!("  -o: Output file path (default: a.ja)");
    return;
  }

  if args.has_arg("-v") {
    return;
  }

  let assemble_mode: bool = args.has_arg("-a");

  let buffer_mode: bool = args.has_arg("-b");

  let measure_time: bool = args.has_arg("-m");

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

  match args.get_arg("-o") {
    Some(arg) => {
      output_file = arg.value.to_string();
    }
    None => {}
  }

  if input_file == "" {
    println!("Rusty Jello requires an input file to run");
    return;
  }

  if !quiet_mode {
    print!("Reading file... ");
  }

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
    Ok(..) => if !quiet_mode {
      println!("Done.");
    },
    Err(err) => {
      println!("Error reading file, {:?}", err);
      return;
    }
  }

  let mut assembler: Assembler = Assembler::new();
  let bytecode: Vec<u8>;

  if !quiet_mode {
    print!("Assembling file... ");
  }

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

  let assembly_duration: Duration = assembly_start_time.elapsed();
  let assembly_elapsed: f64 =
    assembly_duration.as_secs() as f64 + assembly_duration.subsec_nanos() as f64 * 1e-9;

  if !quiet_mode {
    println!("Done.");
  }

  if measure_time {
    println!("Assembly took {:.8} seconds", assembly_elapsed);
  }

  if !assemble_mode {
    if !quiet_mode {
      print!("Loading bytecode into virtual machine... ");
    }

    let mut machine: Machine = Machine::new();
    machine.clock_speed_hz = tick_rate;

    let mut pointer: usize = 0;
    for byte in bytecode {
      machine.memory[pointer] = byte;
      pointer += 1;
    }

    if !quiet_mode {
      println!("Done.");
    }

    if !quiet_mode {
      println!("Running virtual machine until halt... ");
    }

    if !quiet_mode {
      println!("Initial {:?}", machine);
      if !buffer_mode {
        println!();
        println!("---Program Output Start---")
      };
    }
    let execution_start_time = Instant::now();

    let mut buffered_output: String = String::new();

    let mut stdout = io::stdout();

    while !machine.flags.halt {
      if debug_level > 1 {
        println!("{:?}", machine);
      }
      if debug_level > 0 {
        println!("{}", machine.format_inst());
      }
      machine.step();
      if machine.output_buffer.has_bytes(){
        for byte in machine.output_buffer.take_all() {
          if buffer_mode {
            buffered_output.push(byte as char);
          }else{
            write!(stdout, "{}", byte as char).unwrap();
            if debug_level > 0 {
              write!(stdout, "\n").unwrap();
            }
            stdout.flush().unwrap();
          }
        }
      }
    }

    let execution_duration: Duration = execution_start_time.elapsed();
    let execution_elapsed: f64 =
      execution_duration.as_secs() as f64 + execution_duration.subsec_nanos() as f64 * 1e-9;
    if !quiet_mode {
      if !buffer_mode {
        println!();
        println!("----Program Output End----");
      }
      println!();
      println!("Final {:?}", machine);
    }

    if measure_time {
      println!("Execution took {:.8} seconds", execution_elapsed);
    }

    if buffer_mode {
      println!();
      println!("---Program Output Start---");
      print!("{}", buffered_output);
      println!();
      println!("---Program Output Start---");
      println!();
    }

    if !quiet_mode {
      println!("Done.");
    }
  } else {
    let mut out_file: File;
    let output_file_path: &Path = Path::new(&output_file);

    match File::create(output_file_path) {
      Ok(_file) => out_file = _file,
      Err(err) => {
        match err.kind() {
          ErrorKind::PermissionDenied => println!("Output file access denied"),
          _ => println!("Error opening file, {:?}", err),
        }
        return;
      }
    }

    if !quiet_mode {
      print!("Writing file... ");
    }

    match out_file.write(&bytecode) {
      Ok(..) => if !quiet_mode {
        println!("Done.");
      },
      Err(err) => {
        println!("Error writing file, {:?}", err);
        return;
      }
    }
  }
}
