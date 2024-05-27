use std::{
    cell::RefCell,
    env, fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::lox::{entities::eval::Interpreter, interpreter::reader::repl};

fn traverse<F: FnOnce(Rc<RefCell<Interpreter>>, &str) -> bool + Copy>(
    source: &PathBuf,
    inspect: F,
) {
    for file in fs::read_dir(source).unwrap() {
        let interpreter = Rc::new(RefCell::new(Interpreter::new()));
        let str = fs::read_to_string(file.unwrap().path()).unwrap();

        assert!(&inspect(interpreter, &str));
    }
}

#[test]
fn e2e() {
    let mut cwd = env::current_dir().unwrap();
    let prefix = Path::new(file!().strip_suffix("runner.rs").unwrap());
    cwd.pop();
    let folder = cwd.join(prefix);

    traverse(&folder.join("pass/"), |i, input| repl(i, input).is_ok());
    traverse(&folder.join("fail/"), |i, input| repl(i, input).is_err());
}
