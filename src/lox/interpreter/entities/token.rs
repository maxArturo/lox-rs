use super::{val::Literal, TokenType};

#[derive(Debug, Clone)]
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
}
