use loxrs_entities::val::{Function, Value};
use loxrs_types::Result;

use super::eval::Interpreter;

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
