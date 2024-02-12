use std::env;
use std::io;
use std::process::exit;

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
    scan(filename);
}

fn with_prompt() {

    println!("This is the LOX interpreter.");
    println!("Enter statements separated by ENTER.");
    print!("> ");

    let mut statement = String::new();

    io::stdin()
        .read_line(&mut statement)
        .expect("failed to read statement");
    scan(&statement);
}

fn scan(raw_s: &String) {
    println!("received: {raw_s}");
}
