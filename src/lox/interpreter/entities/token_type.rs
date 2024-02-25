use core::fmt;
use std::f64 ;

#[derive(Debug, PartialEq, Clone)]
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
    Identifier(String),

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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_val = match self {
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::Minus => "-",
            Self::Plus => "+",
            Self::SemiColon => ";",
            Self::Slash => "/",
            Self::Star => "*",
            Self::Bang => "!",
            Self::BangEqual => "!=",
            Self::Equal => "=",
            Self::EqualEqual => "==",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::String(str) => str,
            Self::Identifier(str) => str,
            Self::And => "And",
            Self::Class => "Class",
            Self::Else => "Else",
            Self::False => "False",
            Self::Fun => "Fun",
            Self::For => "For",
            Self::If => "If",
            Self::Nil => "Nil",
            Self::Or => "Or",
            Self::Print => "Print",
            Self::Return => "Return",
            Self::Super => "Super",
            Self::This => "This",
            Self::True => "True",
            Self::Var => "Var",
            Self::While => "While",
            Self::Eof => "Eof",
            Self::Number(num) => {
                return write!(f, "{}", num);
            }
        };

        write!(f, "{}", str_val)
    }
}
