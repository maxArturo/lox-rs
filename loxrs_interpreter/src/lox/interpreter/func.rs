use loxrs_types::Result;

use crate::lox::entities::{eval::Interpreter, func::Func, Value};

impl Func {
    pub fn call(&mut self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
        let env = interpreter.env();
        env.borrow_mut().open_scope();

        match self {
            Func::Lox(e) => {
                for (name, val) in e.def.as_ref().params.iter().zip(args.iter()) {
                    env.borrow_mut()
                        .define(name.extract_identifier_str().unwrap(), val.clone());
                }
                interpreter.block_stmt(&e.def.body, env).map(|el| match el {
                    Some(val) => val,
                    None => Value::Nil,
                })
            }
            Func::Native(e) => {
                for (name, val) in e.params.iter().zip(args.iter()) {
                    env.borrow_mut().define(name, val.clone());
                }

                (e.def)(interpreter, env)
            }
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Func::Lox(e) => e.arity(),
            Func::Native(e) => e.arity(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Func::Lox(e) => e.name(),
            Func::Native(e) => e.name(),
        }
    }
}
