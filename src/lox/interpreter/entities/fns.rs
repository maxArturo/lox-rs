use crate::lox::interpreter::eval::Interpreter;

use super::super::error::Result;
use super::val::{Function, Value};

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value>;

    fn arity(&self) -> usize;

    fn name(&self) -> u32;
}

impl Callable for Function {
    // add code here
}
