//! Rusty Jello is a simple (and a bit hack-y) language created by Jacob Allen

use std::env;

pub mod args;
//use std::fs::File;
//use std::io::prelude::*;

fn main() {
  println!("Rusty Jello v0.7.0 (Rumbustious) by Jacob Allen");

  let args: args::Args = args::Args::new(
    env::args().collect(),
    vec!["-v".to_string(), "-h".to_string()],
  );
  let mut input_file: String = "".to_string();
  let debug_level: u8;

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

  println!("{}", debug_level);
  println!("{}", input_file);

  if input_file == "" {
    println!("Rusty Jello requires an input file to run");
    return;
  }
}
