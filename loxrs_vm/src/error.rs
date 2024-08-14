use std::fmt::Display;

use thiserror::Error;

pub type Result<T, U = Error> = std::result::Result<T, U>;

#[derive(Debug, Error)]
pub enum Error {
    OverflowError(OverflowError),
}

#[derive(Debug, Error)]
pub enum OverflowError {
    #[error("exceeded max amount of constants ({0}) in a scope")]
    ExceedsConstSize(usize),
}

// pre-span solution just display
// TODO update when a spanning solution is implemented
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

macro_rules! from_err {
    ($($err:tt),+) => {$(
        impl From<$err> for Error {
            fn from(e: $err) -> Error{
                Error::$err(e)
            }
        })+
    };
}

from_err!(OverflowError);
