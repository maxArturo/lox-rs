use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

use super::class::Instance;
use super::func::Func;

/// Holds lox literal values
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
    Func(Func),
    Instance(Rc<RefCell<Instance>>),
    Nil,
}

/// Holds rust-land computed values from lox expressions and literals
pub type Value = Literal;

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            Self::String(str) => str,
            // TODO and also make it impossiblel to ovewrite native fns
            Self::Func(func) => match func {
                Func::Lox(_) => return write!(f, "[<function>{}]", func.name()),
                Func::Native(_) => return write!(f, "[<native fn>{}]", func.name()),
                Func::Class(class) => return write!(f, "{}", class),
            },
            Self::Number(num) => return write!(f, "{}", num),
            Self::Nil => "Nil",
            Self::Boolean(bool) => {
                return write!(f, "{}", bool);
            }
            Self::Instance(instance) => {
                return write!(f, "{}", instance.borrow());
            }
        };
        write!(f, "{}", str_val)
    }
}
