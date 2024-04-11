use std::fmt::Debug;
use std::fmt::Display;

use super::func::Func;

/// Holds lox literal values
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
    Func(Func),
    Nil,
}

/// Holds rust-land computed values from lox expressions and literals
pub type Value = Literal;

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            Self::String(str) => str,
            Self::Func(func) => return write!(f, "[<function>{}]", func.name()),
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
