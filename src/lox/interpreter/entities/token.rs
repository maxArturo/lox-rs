use super::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    pub lexeme: String,
    line: i32,
    column: i32,
}

impl Token {
    pub fn new(token_type: TokenType, line: i32, column: i32) -> Self {
        let lexeme = token_type.val();
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}
