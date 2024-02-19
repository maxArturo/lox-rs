use std::f64;

#[derive(Debug)]
pub enum TokenType {
    // one char
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // one or two chars
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    String(String),
    // numbers in Lox are double floats
    Number(f64),
    Identifier,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl TokenType {
    // show some representation of the enum
    pub fn val(&self) -> String {
        match self {
            Self::LeftParen => "(".to_string(),
            Self::RightParen => ")".to_string(),
            Self::LeftBrace => "{".to_string(),
            Self::RightBrace => "}".to_string(),
            Self::Comma => ",".to_string(),
            Self::Dot => ".".to_string(),
            Self::Minus => "-".to_string(),
            Self::Plus => "+".to_string(),
            Self::SemiColon => ";".to_string(),
            Self::Slash => "/".to_string(),
            Self::Star => "*".to_string(),
            Self::Bang => "!".to_string(),
            Self::BangEqual => "!=".to_string(),
            Self::Equal => "=".to_string(),
            Self::EqualEqual => "==".to_string(),
            Self::Greater => ">".to_string(),
            Self::GreaterEqual => ">=".to_string(),
            Self::Less => "<".to_string(),
            Self::LessEqual => "<=".to_string(),
            Self::String(str) => str.to_string(),
            Self::Number(num) => num.to_string(),
            Self::Identifier => "Identifier".to_string(),
            Self::And => "And".to_string(),
            Self::Class=> "Class".to_string(),
            Self::Else=> "Else".to_string(),
            Self::False => "False".to_string(),
            Self::Fun => "Fun".to_string(),
            Self::For => "For".to_string(),
            Self::If => "If".to_string(),
            Self::Nil => "Nil".to_string(),
            Self::Or => "Or".to_string(),
            Self::Print => "Print".to_string(),
            Self::Return => "Return".to_string(),
            Self::Super => "Super".to_string(),
            Self::This => "This".to_string(),
            Self::True => "True".to_string(),
            Self::Var => "Var".to_string(),
            Self::While => "While".to_string(),
            Self::Eof => "Eof".to_string(),
        }
    }
}
