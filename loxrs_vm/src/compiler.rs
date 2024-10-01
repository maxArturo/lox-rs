use crate::{
    entities::chunk::Chunk,
    error::{LoxErrorS, Result},
    scanner::{Scanner, TokenS},
};

fn scan(source: &str) -> Result<Vec<TokenS>, Vec<LoxErrorS>> {
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

pub fn compile(source: &str) -> Result<Chunk, Vec<LoxErrorS>> {
    let tokens = scan(source)?;

    todo!()
}

struct Parser {
    tokens: std::vec::IntoIter<TokenS>,
    curr: Option<TokenS>,
    prev: Option<TokenS>,
}

impl Parser {
    fn advance(&mut self) -> Option<()> {
        let next = self.tokens.next()?;
        if let Some(prev) = self.curr.replace(next) {
            self.prev.replace(prev);
        };
        Some(())
    }
}
