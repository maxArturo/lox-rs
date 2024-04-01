use std::fmt::Result as fmt_result;
use std::fmt::{Debug, Formatter};
use std::{fmt::Display, rc::Rc};

use loxrs_env::Env;
use loxrs_types::Result;

/// Holds lox literal values
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
    Func(Function),
    Nil,
}

type FuncDefinition = Rc<dyn Fn(Vec<Value>, Rc<Env<Value>>) -> Result<Value>>;
#[derive(Clone)]
pub struct Function {
    arity: u32,
    name: String,
    func: FuncDefinition,
    env: Rc<Env<Value>>,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity
            && self.name == other.name
            && Rc::ptr_eq(&self.func, &other.func)
            && Rc::ptr_eq(&self.env, &other.env)
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt_result {
        f.debug_struct("Function")
            .field("arity", &self.arity)
            .field("name", &self.name)
            .field("func", &"<function>") // Since the function is opaque, we don't print its contents
            .field("env", &"<env>") // Since the env is large, we don't print its contents
            .finish()
    }
}

/// Holds rust-land computed values from lox expressions and literals
pub type Value = Literal;

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            Self::String(str) => str,
            Self::Func(func) => &func.name,
            Self::Number(num) => {
                return write!(f, "{}", num);
            }
            Self::Nil => "Nil",
            Self::Boolean(bool) => {
                return write!(f, "{}", bool);
            }
        };
        write!(f, "{}", str_val)
    }
}
