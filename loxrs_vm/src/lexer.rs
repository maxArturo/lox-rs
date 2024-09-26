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
    LeftParen((usize, usize)),
    #[token(")", update_col)]
    RightParen((usize, usize)),
    #[token("{", update_col)]
    LeftBrace((usize, usize)),
    #[token("}", update_col)]
    RightBrace((usize, usize)),
    #[token(";", update_col)]
    Semicolon((usize, usize)),
    #[token(",", update_col)]
    Comma((usize, usize)),
    #[token(".", update_col)]
    Dot((usize, usize)),
    #[token("-", update_col)]
    Minus((usize, usize)),
    #[token("+", update_col)]
    Plus((usize, usize)),
    #[token("/", update_col)]
    Slash((usize, usize)),
    #[token("*", update_col)]
    Star((usize, usize)),

    // single or double-chars
    #[token("=", update_col)]
    Equal((usize, usize)),
    #[token("<", update_col)]
    Less((usize, usize)),
    #[token(">", update_col)]
    More((usize, usize)),
    #[token("!=", update_col)]
    BangEqual((usize, usize)),
    #[token("==", update_col)]
    EqualEqual((usize, usize)),
    #[token("<=", update_col)]
    LessEqual((usize, usize)),
    #[token(">=", update_col)]
    GreaterEqual((usize, usize)),

    // literals
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", literal)]
    Literal((usize, usize, String)),

    // strings (to be defined separately from string blocks)
    #[regex(r#""[^"\n]*""#, string)]
    String((usize, usize, String)),

    // numbers
    #[regex(r"[0-9]+(\.[0-9]+)?", number)]
    Number((usize, usize, f64)),
}

fn newline(lexer: &mut logos::Lexer<Token>) -> Skip {
    lexer.extras.line += 1;
    lexer.extras.col = 1;
    lexer.extras.line_offset = lexer.span().end;
    Skip
}

fn update_col(lexer: &mut logos::Lexer<Token>) -> (usize, usize) {
    lexer.extras.col = lexer.span().start - lexer.extras.line_offset + 1;
    (lexer.extras.line, lexer.extras.col)
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
    (lexer.extras.line, lexer.extras.col, slice.to_owned())
}

fn string(lexer: &mut logos::Lexer<Token>) -> String {
    update_col(lexer);
    let slice = lexer.slice();

    (
        lexer.extras.line,
        lexer.extras.col,
        slice[1..slice.len() - 1].to_owned(),
    )
}

fn number(lexer: &mut logos::Lexer<Token>) -> Result<(usize, usize, f64), LexerError> {
    update_col(lexer);
    let slice = lexer.slice();
    slice
        .parse()
        .map(|number| (lexer.extras.line, lexer.extras.col, number))
        .or_else(|_| Err(LexerError::InvalidInteger(slice.to_owned())))
}

// #[cfg(test)]
// mod tests {
//     use pretty_assertions::assert_eq;

//     use super::*;

//     #[test]
//     fn lex_invalid_token() {
//         let exp = vec![
//             Err((
//                 Error::SyntaxError(SyntaxError::UnexpectedInput {
//                     token: "@foo".to_string(),
//                 }),
//                 0..4,
//             )),
//             Ok((5, Token::Identifier("bar".to_string()), 8)),
//         ];
//         let got = Lexer::new("@foo bar").collect::<Vec<_>>();
//         assert_eq!(exp, got);
//     }

//     #[test]
//     fn lex_unterminated_string() {
//         let exp = vec![Err((
//             Error::SyntaxError(SyntaxError::UnterminatedString),
//             0..5,
//         ))];
//         let got = Lexer::new("\"\nfoo").collect::<Vec<_>>();
//         assert_eq!(exp, got);
//     }
// }
