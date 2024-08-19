use std::fmt::Display;

use thiserror::Error;

use codespan_reporting::files::SimpleFiles;

pub type Result<T, U = LoxError> = std::result::Result<T, U>;

#[derive(Debug, Error)]
pub enum LoxError {
    OverflowError(OverflowError),
}

#[derive(Debug, Error)]
pub enum OverflowError {
    #[error("exceeded max amount of constants ({0}) in a scope")]
    ExceedsConstSize(usize),
}

// pre-span solution just display
// TODO update when a spanning solution is implemented
impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

macro_rules! from_err {
    ($($err:tt),+) => {$(
        impl From<$err> for LoxError {
            fn from(e: $err) -> LoxError{
                LoxError::$err(e)
            }
        })+
    };
}

from_err!(OverflowError);
