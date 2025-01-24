use std::{error::Error, fmt::Display};

use crate::data_types::scaler::Scalar;
pub type InterpretResult<T> = Result<T, InterpretError>;

#[derive(Debug)]
pub enum InterpretError {
    Other,
    Runtime(String),
    Return(Scalar),
}
impl InterpretError {
    pub fn rt<T: AsRef<str>>(msg: T) -> Self {
        Self::Runtime(msg.as_ref().to_string())
    }
}
impl Error for InterpretError {}

impl Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[macro_export]
macro_rules! InterpretRtErr {
    (;$x:expr) => {
        Err(InterpretError::rt($x))
    };
}
#[macro_export]
macro_rules! InterpretRet {
    ($x:expr) => {
        Err(InterpretError::Return($x))
    };
}
