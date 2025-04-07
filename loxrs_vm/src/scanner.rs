use std::cmp::Eq;
use std::fmt;
use std::hash::{Hash, Hasher};

use logos::{FilterResult, Logos};

use crate::{
    error::{LoxErrorS, ScannerError},
    types::Span,
};

#[derive(Debug, Logos, PartialEq, Clone)]
#[logos(error = ScannerError)]
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

    // reserved keywords
    #[token("and")]
    And,
    #[token("class")]
    Class,
    #[token("else")]
    Else,
    #[token("false")]
    False,
    #[token("fun")]
    Fun,
    #[token("for")]
    For,
    #[token("if")]
    If,
    #[token("nil")]
    Nil,
    #[token("or")]
    Or,
    #[token("print")]
    Print,
    #[token("return")]
    Return,
    #[token("super")]
    Super,
    #[token("this")]
    This,
    #[token("true")]
    True,
    #[token("var")]
    Var,
    #[token("while")]
    While,

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

impl Eq for Token {}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Token::Number(n) => {
                // Hash the bits of the f64 directly if it's not NaN
                if n.is_nan() {
                    // Hash a specific value for NaN
                    state.write_u64(u64::MAX);
                } else {
                    state.write_u64(n.to_bits());
                }
            }
            Token::String(s) => s.hash(state),
            Token::Literal(s) => s.hash(state),
            // For all other variants, we can use their discriminant
            _ => std::mem::discriminant(self).hash(state),
        }
    }
}

fn literal(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice.to_owned()
}

fn string(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();

    slice[1..slice.len() - 1].to_owned()
}

fn number(lexer: &mut logos::Lexer<Token>) -> Result<f64, ScannerError> {
    let slice = lexer.slice();
    slice
        .parse()
        .or_else(|_| Err(ScannerError::InvalidNumber(slice.to_owned())))
}

fn multiline_comment(lex: &mut logos::Lexer<Token>) -> FilterResult<(), ScannerError> {
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
    FilterResult::Error(ScannerError::MalformedComment)
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
            Token::And => write!(f, "and"),
            Token::Class => write!(f, "Class"),
            Token::Else => write!(f, "Else"),
            Token::False => write!(f, "False"),
            Token::Fun => write!(f, "Fun"),
            Token::For => write!(f, "For"),
            Token::If => write!(f, "If"),
            Token::Nil => write!(f, "Nil"),
            Token::Or => write!(f, "Or"),
            Token::Print => write!(f, "Print"),
            Token::Return => write!(f, "Return"),
            Token::Super => write!(f, "Super"),
            Token::This => write!(f, "This"),
            Token::True => write!(f, "True"),
            Token::Var => write!(f, "Var"),
            Token::While => write!(f, "While"),
        }?;
        write!(f, ">")
    }
}

pub type TokenS = Span<Token>;

pub struct Scanner<'a> {
    matcher: logos::Lexer<'a, Token>,
    curr: Option<TokenS>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            matcher: Token::lexer(source),
            curr: None,
        }
    }
}

impl Iterator for Scanner<'_> {
    type Item = Result<TokenS, Span<ScannerError>>;

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
                                ScannerError::MalformedString(slice.to_owned()),
                                span,
                            )));
                        }

                        Some(Err((err, span)))
                    }
                }
            }
            None => None,
        }
    }
}

pub fn scan(source: &str) -> Result<Vec<TokenS>, Vec<LoxErrorS>> {
    let scanner = Scanner::new(source);
    let mut tokens = vec![];
    let mut errs = vec![];
    for el in scanner.into_iter() {
        match el {
            Ok(token) => tokens.push(token),
            Err(err) => {
                errs.push((err.0.into(), err.1).into());
            }
        }
    }
    if errs.is_empty() {
        return Ok(tokens);
    }
    Err(errs)
}
