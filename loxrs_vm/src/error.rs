use std::num::ParseIntError;

use thiserror::Error;

use crate::span::Spanned;

pub type Result<T, U = LoxError> = std::result::Result<T, U>;

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("OverflowError: {0}")]
    OverflowError(OverflowError),
    #[error("InternalError: {0}")]
    InternalError(InternalError),
    #[error("InvalidAccessError: {0}")]
    InvalidAccessError(InvalidAccessError),
    #[error("ConversionError: {0}")]
    ConversionError(ConversionError),
    #[error("LexerError: {0}")]
    LexerError(LexerError),
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum TokenError {
    #[error("Unrecognized input at line: {0}, col: {1}")]
    UnrecognizedInput(usize, usize),
    #[error("Invalid number at line: {0}, col: {1}")]
    InvalidInteger(usize, usize),
    #[error("Malformed comment at line: {0}, col: {1}")]
    MalformedComment(usize, usize),
}

impl Spanned for TokenError {
    fn span(&self) -> std::ops::Range<usize> {
        match self {
            Self::InvalidInteger(line,col )
            
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum LexerError {
    #[error("Unrecognized input: {0}")]
    UnrecognizedInput(String),
    #[error("Could not recognize {0} as a valid number")]
    InvalidInteger(String),
    #[error("Malformed comment: {0}")]
    MalformedComment(String),
}

impl Default for LexerError {
    fn default() -> Self {
        Self::UnrecognizedInput("Unknown input found".to_owned())
    }
}

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for LexerError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexerError::InvalidInteger("overflow error".to_owned()),
            _ => LexerError::UnrecognizedInput("other error".to_owned()),
        }
    }
}

#[derive(Debug, Error)]
pub enum OverflowError {
    #[error("exceeded max amount of constants ({0}) in a scope")]
    ExceedsConstSize(usize),
    #[error("Index overflow exceeds ({0})")]
    IndexOverflow(usize),
}

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Invalid conversion to: {0}")]
    ConversionError(String),
}

#[derive(Debug, Error)]
pub enum InvalidAccessError {
    #[error("Attempted to use an empty stack")]
    StackEmpty,
}

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("unknown OP provided: (0x{0:x})")]
    UnknownOperation(u8),
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

from_err!(
    OverflowError,
    InternalError,
    ConversionError,
    InvalidAccessError,
    LexerError
);
