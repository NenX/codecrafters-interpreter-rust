

use crate::token_type::{CmpTokenType, TokenType};

#[derive(Clone, Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub lexeme: String,
    // pub literal: Option<String>,
    pub line: usize,
}
impl CmpTokenType<Token> for Token {
    fn is_same_type(&self, right: &Token) -> bool {
        self.t_type.is_same_type(&right.t_type)
    }
}
impl CmpTokenType<TokenType> for Token {
    fn is_same_type(&self, right: &TokenType) -> bool {
        self.t_type.is_same_type(right)
    }
}
