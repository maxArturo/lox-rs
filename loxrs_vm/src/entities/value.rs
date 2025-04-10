use crate::error::{ConversionError, LoxError, Result};
use std::{fmt::Display, mem};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value(u64);

// ensure values are 8 bytes long
const _: () = assert!(mem::size_of::<Value>() == 8);

/// Values in lox are represented as [u64] consts
impl Value {
    // const SIGN_BIT: u64 = 0x8000000000000000;
    const QNAN_BIT: u64 = 0x7FFC000000000000;

    // pub const NIL: Self = Self(Self::QNAN_BIT | 0x1);
    pub const TRUE: Self = Self(Self::QNAN_BIT | 0x2);
    pub const FALSE: Self = Self(Self::QNAN_BIT | 0x3);

    pub fn is_number(&self) -> bool {
        (self.0 & Self::QNAN_BIT) != Self::QNAN_BIT
    }

    fn is_bool(&self) -> bool {
        match self.0 {
            val if val == Self::TRUE.0 => true,
            val if val == Self::FALSE.0 => true,
            _ => false,
        }
    }

    fn get_bool(&self) -> Option<bool> {
        match self.0 {
            val if val == Self::TRUE.0 => Some(true),
            val if val == Self::FALSE.0 => Some(false),
            _ => None,
        }
    }

    fn as_bool(&self) -> bool {
        match self.0 {
            val if val == Self::TRUE.0 => true,
            _ => false,
        }
    }

    pub fn try_bool(&self) -> Result<bool, LoxError> {
        match self.get_bool() {
            Some(val) => Ok(val),
            None => Err(ConversionError::ConversionError("bool".to_owned()).into()),
        }
    }

    fn as_number(&self) -> f64 {
        f64::from_bits(self.0)
    }

    pub fn try_number(&self) -> Result<f64, LoxError> {
        if self.is_number() {
            return Ok(self.as_number());
        }
        Err(ConversionError::ConversionError("number".to_owned()).into())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self(value.to_bits())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        match value {
            true => Self::TRUE,
            false => Self::FALSE,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_number() {
            return write!(f, "{}", self.as_number());
        }

        if self.is_bool() {
            return write!(f, "{}", self.as_bool());
        }

        write!(f, "{}", self.0)
    }
}
