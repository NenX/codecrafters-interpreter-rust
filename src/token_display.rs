use std::fmt::Display;

use crate::{token::Token, token_type::TokenType};
use TokenType::*;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lexeme = String::from_utf8(self.lexeme.to_vec()).unwrap();
        match &self.token_type {
            IDENTIFIER(s) => write!(f, "IDENTIFIER {} null", lexeme,),
            STRING(s) => write!(f, "STRING {} {}", lexeme, s),
            NUMBER(d) => write!(f, "NUMBER {} {:?}", lexeme, d),
            LeftParen => write!(f, "LEFT_PAREN {} null", lexeme),
            RightParen => write!(f, "RIGHT_PAREN {} null", lexeme),
            LeftBrace => write!(f, "LEFT_BRACE {} null", lexeme),
            RightBrace => write!(f, "RIGHT_BRACE {} null", lexeme),
            COMMA => write!(f, "COMMA {} null", lexeme),
            DOT => write!(f, "DOT {} null", lexeme),
            MINUS => write!(f, "MINUS {} null", lexeme),
            PLUS => write!(f, "PLUS {} null", lexeme),
            SEMICOLON => write!(f, "SEMICOLON {} null", lexeme),
            SLASH => write!(f, "SLASH {} null", lexeme),
            STAR => write!(f, "STAR {} null", lexeme),
            BANG => write!(f, "BANG {} null", lexeme),
            BangEqual => write!(f, "BANG_EQUAL {} null", lexeme),
            EQUAL => write!(f, "EQUAL {} null", lexeme),
            EqualEqual => write!(f, "EQUAL_EQUAL {} null", lexeme),
            GREATER => write!(f, "GREATER {} null", lexeme),
            GreaterEqual => write!(f, "GREATER_EQUAL {} null", lexeme),
            LESS => write!(f, "LESS {} null", lexeme),
            LessEqual => write!(f, "LESS_EQUAL {} null", lexeme),
            AND => write!(f, "AND {} null", lexeme),
            CLASS => write!(f, "CLASS {} null", lexeme),
            ELSE => write!(f, "ELSE {} null", lexeme),
            FALSE => write!(f, "FALSE {} null", lexeme),
            FUN => write!(f, "FUN {} null", lexeme),
            FOR => write!(f, "FOR {} null", lexeme),
            IF => write!(f, "IF {} null", lexeme),
            NIL => write!(f, "NIL {} null", lexeme),
            OR => write!(f, "OR {} null", lexeme),
            PRINT => write!(f, "PRINT {} null", lexeme),
            RETURN => write!(f, "RETURN {} null", lexeme),
            SUPER => write!(f, "SUPER {} null", lexeme),
            THIS => write!(f, "THIS {} null", lexeme),
            TRUE => write!(f, "TRUE {} null", lexeme),
            VAR => write!(f, "VAR {} null", lexeme),
            WHILE => write!(f, "WHILE {} null", lexeme),
            EOF => write!(f, "EOF {} null", lexeme),
        }
    }
}
#[test]
fn tt(){
    let f = 1f64;
    println!("{:?}",f)
}