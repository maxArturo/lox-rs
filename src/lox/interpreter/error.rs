use std::fmt;

use super::entities::Token;

pub type Result<T> = std::result::Result<T, LoxErr>;

#[derive(Debug)]
pub enum LoxErr {
    Undefined {
        message: String,
    },
    Eval {
        expr: String,
        message: String,
    },
    Internal {
        message: String,
    },
    Parse {
        token: Token,
        message: String,
    },
    Scan {
        line: i32,
        col: i32,
        message: String,
    },
}

impl fmt::Display for LoxErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: String = match self {
            Self::Parse { token, message } => format!(
                "Parsing error for token: {}\nat line: {}, col: {}",
                message, token.line, token.column
            ),
            Self::Eval { expr, message } => {
                format!("Eval error: {}\n for expression of type: {}", message, expr)
            }
            Self::Scan { line, col, message } => {
                format!("Syntax error: {}\nat line: {}, col: {}", message, line, col)
            }
            Self::Internal { message } => {
                format!("Internal program error: {}", message)
            }
            Self::Undefined { message } => {
                format!("Undefined error: {}", message)
            }
        };
        write!(f, "{}", msg)
    }
}
