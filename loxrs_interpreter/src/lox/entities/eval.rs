use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::{cell::RefCell, fmt::Display};

use super::{Expr, Value};
use loxrs_env::Scope;

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub scope: Rc<Scope<Value>>,
    pub locals: RefCell<HashMap<Expr, usize>>,
}

impl Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interpreter: <locals: [")?;

        for (k, v) in self.locals.borrow().iter() {
            write!(f, "({}: {}) ", k, v)?;
        }

        write!(f, "]")?;

        write!(f, ">")
    }
}
