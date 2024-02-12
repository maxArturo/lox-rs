use std::io;
use std::io::Write;
use std::process::exit;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => with_prompt(),
        2 => {
            let filename = &args[1];
            with_file(filename);
        }
        _ => {
            println!("USAGE: lox-rs [name of file]");
            exit(1);
        }
    }
}

fn with_file(filename: &String) {
    println!("you provided a file: {filename}.");

    match fs::read_to_string(filename) {
        Ok(str) => scan(&str),
        Err(e) => {
            println!("Error reading file: {e}");
            exit(1);
        }
    }
}

fn with_prompt() {
    println!("This is the LOX interpreter.");
    println!("Enter statements separated by ENTER.");
    println!("EXIT with CTRL-D.");

    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut statement = String::new();

        match io::stdin().read_line(&mut statement) {
            Ok(0) => break,
            Ok(str) => str,
            Err(e) => {
                println!("Unrecognized statement: {e}");
                continue;
            }
        };

        scan(&statement);
    }
}

fn scan(raw_s: &String) {
    println!("received: {raw_s}");
}
