use std::{
    cell::RefCell,
    env, fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::lox::{entities::eval::Interpreter, interpreter::reader::repl};

fn traverse<
    F: FnOnce(std::result::Result<(), std::vec::Vec<loxrs_types::LoxErr>>) -> bool + Copy,
>(
    source: &PathBuf,
    inspect: F,
) {
    for file in fs::read_dir(source).unwrap() {
        println!("Testing file: {:?}", file);
        let interpreter = Rc::new(RefCell::new(Interpreter::new()));
        let str = fs::read_to_string(file.unwrap().path()).unwrap();
        let res = repl(interpreter, &str);
        println!("testing output: {:?}", &res);
        assert!(inspect(res));
    }
}

fn get_test_folder() -> std::path::PathBuf {
    let mut cwd = env::current_dir().unwrap();
    let prefix = Path::new(file!().strip_suffix("runner.rs").unwrap());
    cwd.pop();
    cwd.join(prefix)
}

#[test]
fn e2e_pass() {
    let folder = get_test_folder();
    traverse(&folder.join("pass/"), |res| res.is_ok());
}

#[test]
fn e2e_fail() {
    let folder = get_test_folder();
    traverse(&folder.join("fail/"), |res| res.is_err());
}

#[test]
fn spec_samples() {
    let folder = get_test_folder();
    traverse(&folder.join("samples/"), |res| res.is_ok());
}

#[test]
fn spec_assignment() {
    let folder = get_test_folder();
    traverse(&folder.join("spec/assignment"), |res| res.is_ok());
}

#[test]
fn spec_block() {
    let folder = get_test_folder();
    traverse(&folder.join("spec/block"), |res| res.is_ok());
}

#[test]
fn spec_bool() {
    let folder = get_test_folder();
    traverse(&folder.join("spec/bool"), |res| res.is_ok());
}

#[test]
fn spec_call() {
    let folder = get_test_folder();
    traverse(&folder.join("spec/call"), |res| res.is_err());
}

#[test]
fn spec_closure() {
    let folder = get_test_folder();
    traverse(&folder.join("spec/closure"), |res| res.is_ok());
}
