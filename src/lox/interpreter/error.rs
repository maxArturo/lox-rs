use std::fmt;

use super::entities::Token;

pub type Result<T> = std::result::Result<T, LoxErr>;

#[derive(Debug, Clone)]
pub enum LoxErr {
    ParseError { token: Token, message: String },
    ScanError { line: i32, col: i32, message: String },
}

impl fmt::Display for LoxErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: String = match self {
            Self::ParseError { token, message } => format!(
                "Parsing error for token: {}\nat line: {}, col: {}",
                message, token.line, token.column
            ),

            Self::ScanError { line, col, message} => {
                format!("Syntax error: {}\nat line: {}, col: {}", message, line, col)
            }
        };
        write!(f, "{}", msg)
    }
}
