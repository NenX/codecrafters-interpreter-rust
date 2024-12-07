use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::token_type::TokenType;

lazy_static! {
    pub static ref keywords_map: HashMap<String, TokenType> = {
        [
            ("and", TokenType::AND),
            ("class", TokenType::CLASS),
            ("else", TokenType::ELSE),
            ("false", TokenType::FALSE),
            ("for", TokenType::FOR),
            ("fun", TokenType::FUN),
            ("if", TokenType::IF),
            ("nil", TokenType::NIL),
            ("or", TokenType::OR),
            ("print", TokenType::PRINT),
            ("return", TokenType::RETURN),
            ("super", TokenType::SUPER),
            ("this", TokenType::THIS),
            ("true", TokenType::TRUE),
            ("var", TokenType::VAR),
            ("while", TokenType::WHILE),
        ]
        .iter()
        .map(|x| (x.0.to_string(), x.1.clone()))
        .collect()
    };
}
