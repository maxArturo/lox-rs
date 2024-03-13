use super::reader::{run_file, run_prompt};

use std::env;
use std::process::exit;

pub fn read_input() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => {
            let filename = &args[1];
            run_file(filename);
        }
        _ => {
            println!("USAGE: lox-rs [name of file]");
            exit(64); // EX_USAGE error code
        }
    }
}
