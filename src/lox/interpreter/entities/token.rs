use super::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    line: i32,
    column: i32,
}

impl Token {
    pub fn new(token_type: TokenType, line: i32, column: i32) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }
}
