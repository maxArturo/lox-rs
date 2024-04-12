use std::fmt::Debug;
use std::rc::Rc;

use super::Value;
use loxrs_env::Scope;

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub scope: Rc<Scope<Value>>,
}
