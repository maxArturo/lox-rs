use crate::lox::interpreter::eval::Interpreter;
use loxrs_entities::Result;

use super::val::{Function, Value};

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value>;

    fn arity(&self) -> usize;

    fn name(&self) -> u32;
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value> {
        todo!()
    }

    fn arity(&self) -> usize {
        todo!()
    }

    fn name(&self) -> u32 {
        todo!()
    }
}
