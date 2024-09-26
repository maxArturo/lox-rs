use logos::{FilterResult, Lexer, Logos, Skip};

use crate::error::LexerError;

pub struct Extras {
    pub line: usize,
    line_offset: usize,
    pub col: usize,
}

impl Default for Extras {
    fn default() -> Self {
        Self {
            line: 1,
            col: 1,
            line_offset: 0,
        }
    }
}

#[derive(Debug, Logos, PartialEq)]
#[logos(error = LexerError)]
#[logos(skip r"[ \t]+")]
#[logos(extras = Extras)]

pub enum Token {
    #[regex(r"[\n\r\f]", newline)]
    Newline,
    #[regex(r"\/\/[^\n]*", logos::skip)]
    Comment,


    #[token("/*", block_comment)]
    BlockComment,
    // single chars
    #[token("(", update_col)]
    LeftParen,
    #[token(")", update_col)]
    RightParen,
    #[token("{", update_col)]
    LeftBrace,
    #[token("}", update_col)]
    RightBrace,
    #[token(";", update_col)]
    Semicolon,
    #[token(",", update_col)]
    Comma,
    #[token(".", update_col)]
    Dot,
    #[token("-", update_col)]
    Minus,
    #[token("+", update_col)]
    Plus,
    #[token("/", update_col)]
    Slash,
    #[token("*", update_col)]
    Star,

    // single or double-chars
    #[token("=", update_col)]
    Equal,
    #[token("<", update_col)]
    Less,
    #[token(">", update_col)]
    More,
    #[token("!=", update_col)]
    BangEqual,
    #[token("==", update_col)]
    EqualEqual,
    #[token("<=", update_col)]
    LessEqual,
    #[token(">=", update_col)]
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

// fn comment(lexer: &mut logos::Lexer<Token>) -> Skip {
//     lexer.extras.line += 1;
//     lexer.extras.col = 1;
//     lexer.extras.line_offset = lexer.span().end;

//     println!(
//         "yo this is a COMMENT: col: {}  full span: {:?}",
//         lexer.extras.col,
//         lexer.span()
//     );
//     Skip
// }

fn newline(lexer: &mut logos::Lexer<Token>) -> Skip {
    lexer.extras.line += 1;
    lexer.extras.col = 1;
    lexer.extras.line_offset = lexer.span().end;

    println!(
        "yo this is a NEWLINE: col: {}  full span: {:?}",
        lexer.extras.col,
        lexer.span()
    );
    Skip
}

fn update_col(lexer: &mut logos::Lexer<Token>) {
    lexer.extras.col = lexer.span().start - lexer.extras.line_offset + 1;
}

fn block_comment(lex: &mut Lexer<Token>) -> FilterResult<(), LexerError> {
    enum State {
        ExpectStar,
        ExpectSlash,
    }
    let remainder = lex.remainder();
    let (mut state, mut iter) = (State::ExpectStar, remainder.chars());
    while let Some(next_char) = iter.next() {
        match next_char {
            '\n' => {
                lex.extras.line += 1;
                lex.extras.col = 1;
                lex.extras.line_offset = lex.span().end + (remainder.len() - iter.as_str().len());
                state = State::ExpectStar;
            }
            '*' => state = State::ExpectSlash,
            '/' if matches!(state, State::ExpectSlash) => {
                lex.bump(remainder.len() - iter.as_str().len());
                return FilterResult::Skip;
            }
            _ => state = State::ExpectStar,
        }
    }
    lex.bump(remainder.len());
    FilterResult::Error(LexerError::MalformedComment(lex.slice().to_owned()))
}

fn literal(lexer: &mut logos::Lexer<Token>) -> String {
    update_col(lexer);
    let slice = lexer.slice();
    slice.to_owned()
}

fn string(lexer: &mut logos::Lexer<Token>) -> String {
    update_col(lexer);
    let slice = lexer.slice();
    slice[1..slice.len() - 1].to_owned()
}

fn number(lexer: &mut logos::Lexer<Token>) -> Result<f64, LexerError> {
    update_col(lexer);
    let slice = lexer.slice();
    slice
        .parse()
        .or_else(|_| Err(LexerError::InvalidInteger(slice.to_owned())))
}
