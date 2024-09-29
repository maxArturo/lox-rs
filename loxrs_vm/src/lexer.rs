use std::fmt;

use logos::{FilterResult, Logos};

use crate::{error::LexerError, types::Span};

#[derive(Debug, Logos, PartialEq)]
#[logos(error = LexerError)]
#[logos(skip r"[ \t]+")]

pub enum Token {
    #[regex(r"[\n\r\f]", logos::skip)]
    Newline,
    #[regex(r"\/\/[^\n]*", logos::skip)]
    Comment,
    #[token("/*", multiline_comment)]
    BlockComment,

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

    // strings (to be defined separately from string blocks)
    #[regex(r#""[^"\n]*""#, string)]
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
    slice
        .parse()
        .or_else(|_| Err(LexerError::InvalidInteger(slice.to_owned())))
}

fn multiline_comment(lex: &mut logos::Lexer<Token>) -> FilterResult<(), LexerError> {
    enum State {
        ExpectStar,
        ExpectSlash,
    }
    let remainder = lex.remainder();
    let (mut state, mut iter) = (State::ExpectStar, remainder.chars());
    while let Some(next_char) = iter.next() {
        match next_char {
            '*' => state = State::ExpectSlash,
            '/' if matches!(state, State::ExpectSlash) => {
                lex.bump(remainder.len() - iter.as_str().len());
                return FilterResult::Skip;
            }
            _ => state = State::ExpectStar,
        }
    }
    lex.bump(remainder.len());
    FilterResult::Error(LexerError::MalformedComment)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token<")?;
        match self {
            Token::Newline => write!(f, "Newline"),
            Token::Comment => write!(f, "Comment"),
            Token::BlockComment => write!(f, "BlockComment"),
            Token::LeftParen => write!(f, "LeftParen"),
            Token::RightParen => write!(f, "RightParen"),
            Token::LeftBrace => write!(f, "LeftBrace"),
            Token::RightBrace => write!(f, "RightBrace"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Comma => write!(f, "Comma"),
            Token::Dot => write!(f, "Dot"),
            Token::Minus => write!(f, "Minus"),
            Token::Plus => write!(f, "Plus"),
            Token::Slash => write!(f, "Slash"),
            Token::Star => write!(f, "Star"),
            Token::Equal => write!(f, "Equal"),
            Token::Less => write!(f, "Less"),
            Token::More => write!(f, "More"),
            Token::BangEqual => write!(f, "BangEqual"),
            Token::EqualEqual => write!(f, "EqualEqual"),
            Token::LessEqual => write!(f, "LessEqual"),
            Token::GreaterEqual => write!(f, "GreaterEqual"),
            Token::Literal(value) => write!(f, "{}: {:16}", "Literal", value),
            Token::String(value) => write!(f, "{}: {:16}", "String", value),
            Token::Number(value) => write!(f, "{}: {:16}", "Number", value),
        }?;
        write!(f, ">")
    }
}

pub type TokenS = Span<Token>;

pub struct Lexer<'a> {
    matcher: logos::Lexer<'a, Token>,
    curr: Option<TokenS>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            matcher: Token::lexer(source),
            curr: None,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<TokenS, Span<LexerError>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(res) = self.curr.take() {
            return Some(Ok(res));
        }

        match self.matcher.next() {
            Some(res) => {
                let span = self.matcher.span();
                match res {
                    Ok(token) => Some(Ok((token, span))),
                    // narrow down errors
                    Err(err) => {
                        let slice = self.matcher.slice();
                        if slice.starts_with('"') {
                            return Some(Err((
                                LexerError::MalformedString(slice.to_owned()),
                                span,
                            )));
                        }
                        // if slice.starts_with("//") || slice.starts_with("/*") {
                        //     return Some(Err((
                        //         LexerError::MalformedComment,
                        //         span,
                        //     )));
                        // }

                        Some(Err((err, span)))
                    }
                }
            }
            None => None,
        }
    }
}
