use std::num::ParseIntError;

use thiserror::Error;

use crate::types::Span;

pub type LoxErrorS = Span<LoxError>;

pub type Result<T, U = LoxError> = std::result::Result<T, U>;

#[derive(Debug, Error, Clone)]
pub enum LoxError {
    #[error("OverflowError: {0}")]
    OverflowError(OverflowError),
    #[error("InternalError: {0}")]
    InternalError(InternalError),
    #[error("InvalidAccessError: {0}")]
    InvalidAccessError(InvalidAccessError),
    #[error("ConversionError: {0}")]
    ConversionError(ConversionError),
    #[error("ScannerError: {0}")]
    ScannerError(ScannerError),
    #[error("Syntax Error: {0}")]
    SyntaxError(SyntaxError),
    #[error("Compiler Error: {0}")]
    CompilerError(CompilerError),
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ScannerError {
    #[error("Unrecognized input: {0}")]
    UnrecognizedInput(String),
    #[error("Malformed string: {0}")]
    MalformedString(String),
    #[error("Could not recognize {0} as a valid number")]
    InvalidNumber(String),
    #[error("Malformed comment")]
    MalformedComment,
}

impl Default for ScannerError {
    fn default() -> Self {
        Self::UnrecognizedInput("Unrecognized input".to_owned())
    }
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum SyntaxError {
    #[error("Invalid Syntax")]
    InvalidSyntax,
    #[error("{0}")]
    UnexpectedValue(String),
}

impl Default for SyntaxError {
    fn default() -> Self {
        Self::InvalidSyntax
    }
}

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for ScannerError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => ScannerError::InvalidNumber("overflow error".to_owned()),
            _ => ScannerError::UnrecognizedInput("other error".to_owned()),
        }
    }
}

#[derive(Debug, Error, Clone)]
pub enum OverflowError {
    #[error("exceeded max amount of constants ({0}) in a scope")]
    ExceedsConstSize(usize),
    #[error("Index overflow exceeds ({0})")]
    IndexOverflow(usize),
}

#[derive(Debug, Error, Clone)]
pub enum ConversionError {
    #[error("Invalid conversion to: {0}")]
    ConversionError(String),
}

#[derive(Debug, Error, Clone)]
pub enum InvalidAccessError {
    #[error("Attempted to use an empty stack")]
    StackEmpty,
}

#[derive(Debug, Error, Clone)]
pub enum InternalError {
    #[error("unknown OP provided: (0x{0:x})")]
    UnknownOperation(u8),
    #[error("Unexpected code path, programmer error")]
    UnexpectedCodePath,
}

#[derive(Debug, Error, Clone)]
pub enum CompilerError {
    #[error("Expected a different precedence for {0}")]
    PrecedenceError(String),
    #[error("Did not find parse logic for token: {0}")]
    ParseLogicNotFound(String),
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
    CompilerError,
    ConversionError,
    InvalidAccessError,
    ScannerError,
    SyntaxError
);

#[derive(Debug, Clone)]
pub struct Label(pub codespan_reporting::diagnostic::Label<()>);

impl From<LoxErrorS> for Label {
    fn from((err, range): LoxErrorS) -> Self {
        match &err {
            LoxError::ScannerError(lexer_error) => match lexer_error {
                ScannerError::UnrecognizedInput(unrecognized) => Label(
                    codespan_reporting::diagnostic::Label::secondary((), range)
                        .with_message(unrecognized),
                ),
                _ => Label(
                    codespan_reporting::diagnostic::Label::primary((), range)
                        .with_message(err.to_string()),
                ),
            },
            _ => Label(
                codespan_reporting::diagnostic::Label::primary((), range)
                    .with_message(err.to_string()),
            ),
        }
    }
}
