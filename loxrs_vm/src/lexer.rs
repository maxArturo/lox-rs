use logos::{Lexer, Logos, Skip};

use crate::error::LexerError;

#[derive(Debug, Logos, PartialEq)]
#[logos(error = LexerError)]
#[logos(skip r"[ \t\n\r\f]+")]
pub enum Token {
    #[regex(r"//.*\n", logos::skip)]
    #[regex(r"/\*.*\*/", logos::skip)]
    Comment,

    // single chars
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,

    // single or double-chars
    #[token("=")]
    Equal,
    #[token("<")]
    Less,
    #[token(">")]
    More,
    #[token("!=")]
    BangEqual,
    #[token("==")]
    EqualEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,

    // literals
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", literal)]
    Literal(String),

    // strings
    #[regex(r#""[^"]*""#, string)]
    String(String),

    // numbers
    #[regex(r"[0-9]+(\.[0-9]+)?", number)]
    Number(f64),
}

fn literal(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice.to_owned()
}

fn string(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice[1..slice.len() - 1].to_owned()
}

fn number(lexer: &mut logos::Lexer<Token>) -> Result<f64, LexerError> {
    let slice = lexer.slice();
    slice.parse().or_else(|_| {
        Err(LexerError::InvalidInteger(format!(
            "could not recognize {slice} as a valid number"
        )))
    })
}
