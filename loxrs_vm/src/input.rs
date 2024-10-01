use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
    process::exit,
};

use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use log::error;

use crate::{
    error::{Label, LoxError, LoxErrorS},
    vm::VM,
};

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
        let _ = vm
            .interpret(&statement)
            .inspect_err(|errs| report_errors(errs, "REPL input", &statement));
    }
}

fn run_file(filename: &String) {
    println!("you provided a file: {filename}.");

    let mut vm = VM::new();
    match fs::read_to_string(filename) {
        Ok(str) => {
            let _ = vm
                .interpret(&str)
                .inspect_err(|errs| report_errors(errs, filename, &str));
        }
        Err(e) => {
            error!("Error reading file: {e}");
            exit(66); // EX_NOINPUT
        }
    }
}

fn report_errors(errs: &Vec<LoxErrorS>, filename: &str, source: &str) {
    let mut error_map: HashMap<&'static str, Vec<Label>> = HashMap::new();
    for err in errs {
        match &err.0 {
            LoxError::ScannerError(_) => error_map
                .entry("Syntax Error")
                .or_insert(vec![])
                .push((err.0.clone(), err.1.clone()).into()),
            _ => error_map
                .entry("Runtime Error")
                .or_insert(vec![])
                .push((err.0.clone(), err.1.clone()).into()),
        }
    }

    let file = SimpleFile::new(filename, source);
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    for (str, labels) in error_map {
        let diagnostic: Diagnostic<()> = Diagnostic::error()
            .with_message(str)
            .with_labels(labels.iter().map(|el| el.0.clone()).collect());
        term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
    }
}
