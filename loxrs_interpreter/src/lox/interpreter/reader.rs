use log::{error, trace};
use std::cell::RefCell;
use std::io::Write;
use std::process::exit;
use std::rc::Rc;
use std::{fs, io};

use crate::lox::entities::eval::Interpreter;
use crate::lox::interpreter::scan_parse;

use super::resolver::Resolver;

pub fn run_file(filename: &String) {
    println!("you provided a file: {filename}.");

    match fs::read_to_string(filename) {
        Ok(str) => {
            let interpreter = Rc::new(RefCell::new(Interpreter::new()));
            repl(interpreter, &str);
        }
        Err(e) => {
            error!("Error reading file: {e}");
            exit(66); // EX_NOINPUT
        }
    }
}

pub fn run_prompt() {
    println!("This is the LOX interpreter.");
    println!("Enter statements separated by ENTER.");
    println!("EXIT with CTRL-D.");

    let interpreter = Rc::new(RefCell::new(Interpreter::new()));
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
        repl(Rc::clone(&interpreter), &statement);
    }
}

fn repl(interpreter: Rc<RefCell<Interpreter>>, str: &str) {
    let mut resolver = Resolver::new(Rc::clone(&interpreter));

    let _ = scan_parse(str)
        .and_then(|stmts| {
            resolver.resolve(&stmts).map_err(|e| vec![e]).and_then(|_| {
                trace!(
                    "post resolver Interpreter: {}",
                    interpreter.as_ref().borrow()
                );

                interpreter
                    .as_ref()
                    .borrow_mut()
                    .interpret(&stmts[..])
                    .map_err(|e| vec![e])
            })
        })
        .inspect_err(|errs| {
            for e in errs {
                error!("{:?}", e);
            }
        });
}
