use std::rc::Rc;

use log::trace;
use loxrs_types::Result;

use crate::lox::entities::{eval::Interpreter, func::Func, Value};

impl Func {
    pub fn call(&mut self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
        match self {
            Func::Lox(e) => {
                let env = Rc::clone(&e.env);
                env.borrow_mut().open_scope();

                for (name, val) in e.def.as_ref().params.iter().zip(args.iter()) {
                    trace!(
                        "setting assignment before function block call: [{}] = {}",
                        name,
                        val
                    );
                    env.borrow_mut()
                        .define(name.extract_identifier_str().unwrap(), val.clone());
                }
                trace!(
                    "ABOUT TO execute function: <{}>  block call: with env: {}",
                    e.name(),
                    env.borrow()
                );
                let res = interpreter
                    .block_stmt(&e.def.body, Rc::clone(&env))
                    .map(|el| match el {
                        Some(val) => val,
                        None => Value::Nil,
                    });

                env.borrow_mut().close_scope();
                res
            }
            Func::Native(e) => {
                let env = Rc::clone(&e.env);
                env.borrow_mut().open_scope();
                for (name, val) in e.params.iter().zip(args.iter()) {
                    env.borrow_mut().define(name, val.clone());
                }

                let res = (e.def)(interpreter, Rc::clone(&env));
                env.borrow_mut().close_scope();
                res
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
