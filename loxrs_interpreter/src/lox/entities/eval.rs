use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use super::Value;
use loxrs_env::Env;

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub env: Rc<RefCell<Env<Value>>>,
}
