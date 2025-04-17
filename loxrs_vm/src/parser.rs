use log::{debug, trace};

use crate::{
    constants::NO_SPAN,
    error::{InternalError, LoxErrorS, SyntaxError},
    scanner::{Token, TokenS},
};

pub struct Parser {
    tokens: std::vec::IntoIter<TokenS>,
    pub curr: Option<TokenS>,
    pub prev: Option<TokenS>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenS>) -> Self {
        debug!("Extracted tokens: {:?}", tokens);
        let mut tokens = tokens.into_iter();
        let curr = tokens.next();
        Self {
            tokens,
            curr,
            prev: None,
        }
    }

    pub fn advance(&mut self) {
        self.prev = self.curr.take();
        self.curr = self.tokens.next();

        trace!(
            "advanced parser, `prev`: {:?}, `curr`: {:?}",
            self.prev,
            self.curr
        );
    }

    pub fn consume(&mut self, token: Token, err: &str) -> Result<(), LoxErrorS> {
        if let Some(t) = &self.curr {
            if token == t.0 {
                self.advance();
                return Ok(());
            }

            return Err((
                SyntaxError::UnexpectedValue(err.to_owned()).into(),
                t.1.to_owned(),
            ));
        }

        Err((InternalError::UnexpectedCodePath.into(), NO_SPAN))
    }
}
