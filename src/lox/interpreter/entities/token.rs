use super::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
    column: Option<i32>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i32, column: Option<i32>) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}
