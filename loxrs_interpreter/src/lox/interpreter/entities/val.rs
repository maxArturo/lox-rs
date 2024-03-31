use std::fmt::Display;

/// Holds lox literal values
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
    Func(Function),
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    arity: u32,
    name: String,
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
