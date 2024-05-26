use std::rc::Rc;

use log::trace;
use loxrs_env::Scope;
use loxrs_types::Result;

use crate::lox::{
    entities::{eval::Interpreter, func::Func, Value},
    interpreter::visitor::StmtVisitor,
};

impl Func {
    pub fn call(&mut self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
        match self {
            Func::Lox(e) => {
                let scope = Scope::from_parent(Rc::clone(&e.scope));

                for (name, val) in e.def.params.iter().zip(args.iter()) {
                    trace!(
                        "setting assignment before function block call: [{}] = {}",
                        name,
                        val
                    );
                    scope.define(name.extract_identifier_str().unwrap(), val.clone());
                }
                trace!(
                    "ABOUT TO execute function: <{}>  block call: with env: {}",
                    e.name(),
                    scope,
                );
                interpreter
                    .block_stmt(&e.def.body, scope)
                    .map(|el| match el {
                        Some(val) => val,
                        None => Value::Nil,
                    })
            }
            Func::Native(e) => {
                let scope = Scope::from_parent(Rc::clone(&e.scope));
                for (name, val) in e.params.iter().zip(args.iter()) {
                    scope.define(name, val.clone());
                }

                (e.def)(interpreter, scope)
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
