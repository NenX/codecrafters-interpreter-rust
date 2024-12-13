use std::fmt::Display;

use bytes::Bytes;

use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    // pub literal: Option<String>,
    pub line: usize,
}
