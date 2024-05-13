use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use super::{Expr, Value};
use loxrs_env::Scope;

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub scope: Rc<Scope<Value>>,
    pub locals: RefCell<HashMap<Expr, usize>>,
}
