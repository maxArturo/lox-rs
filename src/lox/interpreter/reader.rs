use std::io::Write;
use std::process::exit;
use std::{fs, io};

use super::entities::run_scanner;

pub fn run_file(filename: &String) {
    println!("you provided a file: {filename}.");

    match fs::read_to_string(filename) {
        Ok(str) => run_scanner(&str),
        Err(e) => {
            println!("Error reading file: {e}");
            exit(66); // EX_NOINPUT
        }
    }
}

pub fn run_prompt() {
    println!("This is the LOX interpreter.");
    println!("Enter statements separated by ENTER.");
    println!("EXIT with CTRL-D.");

    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut statement = String::new();

        match io::stdin()
            .read_line(&mut statement) {
            Ok(0) => break,
            Ok(str) => str,
            Err(e) => {
                println!("Unrecognized statement: {e}");
                continue;
            }
        };

        run_scanner(&statement);
    }
}
