use logos::Logos;

use crate::{error::Result, lexer::Token};

pub fn compile(line: &str) -> Result<()> {
    let mut lexer = Token::lexer(line);
    while let Some(res) = lexer.next() {
        match res {
            Ok(token) => {
                println!(
                    "line: {:03}, col: {:03}, start: {:04} end: {:04} word: {:?}",
                    lexer.extras.line,
                    lexer.extras.col,
                    lexer.span().start,
                    lexer.span().end,
                    token
                );

                if let Token::BlockComment = token {
                    println!("yo this is a BLOCK: {:?}", lexer.span())
                }
            }
            Err(err) => {
                println!(
                    "line: {:03}, col: {:03}, start: {:04} end: {:04} ERR: {}",
                    lexer.extras.line,
                    lexer.extras.col,
                    lexer.span().start,
                    lexer.span().end,
                    err
                )
            }
        }
    }

    Ok(())
}
