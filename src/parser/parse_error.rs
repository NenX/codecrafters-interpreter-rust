use std::{error::Error, fmt::Display};

use crate::token::Token;

#[derive(Debug)]
pub enum ParseError {
    NotExpected(Token, String),
}
impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
