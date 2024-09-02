use thiserror::Error;

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
    InvalidAccessError
);
