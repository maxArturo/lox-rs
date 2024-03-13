use core::fmt;

use crate::lox::interpreter::error::LoxErr;

use super::super::error::Result;
use super::{val::Literal, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: i32,
    pub column: i32,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Option<Literal>, line: i32, column: i32) -> Self {
        Token {
            token_type,
            literal,
            line,
            column,
        }
    }

    pub fn ensure_type(&self, ensure: TokenType) -> bool {
        self.token_type == ensure
    }

    pub fn extract_identifier_str(&self) -> Result<&str> {
        let err = || LoxErr::Internal {
            message: "No string value defined for identifier token".to_string(),
        };
        if self.ensure_type(TokenType::Identifier) {
            return self.literal.as_ref().ok_or(err()).and_then(|l| match l {
                Literal::String(str) => Ok(str.as_str()),
                _ => Err(err()),
            });
        }

        Err(err())
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token: type={}, line={}, column={}",
            self.token_type, self.line, self.column
        )?;

        if let Some(literal) = &self.literal {
            write!(f, ", literal={}", literal)?;
        }

        Ok(())
    }
}
