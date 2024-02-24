use super::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: i32,
    pub column: i32,
}

impl Token {
    pub fn new(token_type: TokenType, line: i32, column: i32) -> Self {
        Token {
            token_type,
            line,
            column,
        }
    }
}
