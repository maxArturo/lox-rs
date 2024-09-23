use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use log::error;

use crate::vm::VM;

pub fn read_input() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => {
            run_file(&args[1]);
        }
        _ => {
            println!("USAGE: lox-rs [name of file]");
            exit(64); // EX_USAGE error code
        }
    }
}

fn repl() {
    println!("This is the LOX interpreter.");
    println!("Enter statements separated by ENTER.");
    println!("EXIT with CTRL-D.");

    let mut vm = VM::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut statement = String::new();

        match io::stdin().read_line(&mut statement) {
            Ok(0) => break,
            Ok(str) => str,
            Err(e) => {
                error!("Unrecognized input: {e}");
                continue;
            }
        };
        let _ = vm.interpret(&statement).inspect_err(|err| {
            error!("{:?}", err);
        });
    }
}

fn run_file(filename: &String) {
    println!("you provided a file: {filename}.");

    let mut vm = VM::new();
    match fs::read_to_string(filename) {
        Ok(str) => {
            let _ = vm.interpret(&str).inspect_err(|err| {
                error!("{:?}", err);
            });
        }
        Err(e) => {
            error!("Error reading file: {e}");
            exit(66); // EX_NOINPUT
        }
    }
}
