
use anyhow::Context;
use bytes::{Buf, Bytes};

use crate::{
    constants::keywords_map,
    error::{my_error, unexpected_terminal_err, MyResult},
    token::Token,
    token_type::TokenType,
};
use TokenType::*;
#[derive(Debug, Clone)]
pub struct Scanner {
    source: Bytes,
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: Bytes) -> Self {
        Self {
            source,
            line: 1,
            start: 0,
            current: 0,
            tokens: vec![],
        }
    }
    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
    pub fn print_tokens(&self) {
        for t in &self.tokens {
            println!("{}", t)
        }
    }
    pub fn scan_tokens(&mut self) -> MyResult<()> {
        while !self.is_at_end() {
            self.scan_token()?;
        }
        if self.is_at_end() {
            self.add_token(EOF);
        }
        Ok(())
    }
    fn scan_token(&mut self) -> MyResult<()> {
        let b = self.advance_unchecked();
        let token_type = match b {
            b'(' => LeftParen,
            b')' => RightParen,
            b'{' => LeftBrace,
            b'}' => RightBrace,
            b',' => COMMA,
            b'.' => DOT,
            b'-' => MINUS,
            b'+' => PLUS,
            b';' => SEMICOLON,
            b'*' => STAR,
            b'!' => {
                if self.match_advance(b'=') {
                    BangEqual
                } else {
                    BANG
                }
            }
            b'=' => {
                if self.match_advance(b'=') {
                    EqualEqual
                } else {
                    EQUAL
                }
            }

            b'<' => {
                if self.match_advance(b'=') {
                    LessEqual
                } else {
                    LESS
                }
            }
            b'>' => {
                if self.match_advance(b'=') {
                    GreaterEqual
                } else {
                    GREATER
                }
            }
            b'/' => {
                if self.match_advance(b'/') {
                    while !self.is_at_end() && self.advance_unchecked() != b'\n' {}
                    self.line += 1;
                    self.flush();
                    return Ok(());
                } else {
                    SLASH
                }
            }
            b'\n' => {
                self.line += 1;
                self.flush();
                return Ok(());
            }
            b'"' => {
                let r = self.string();
                match r {
                    Some(t) => t,
                    None => return Ok(()),
                }
            }

            _ if [b' ', b'\t', b'\r'].contains(&b) => {
                self.flush();
                return Ok(());
            }
            _ if b.is_ascii_digit() => self.number()?,
            _ if b.is_ascii_alphabetic() || b == b'_' => self.identifier()?,
            _ => {
                let char = String::from_utf8([b].to_vec()).unwrap();
                my_error(self.line, format!("Unexpected character: {}", char));
                self.flush();
                return Ok(());
            }
        };
        self.add_token(token_type);
        Ok(())
    }

    fn advance_unchecked(&mut self) -> u8 {
        let b = self.source.get(self.current).unwrap();
        self.current += 1;
        *b
    }
    fn match_advance(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        let b = self.peek().unwrap();
        let is_expected = b == expected;
        if is_expected {
            self.current += 1
        }
        is_expected
    }
    fn is_at_end(&self) -> bool {
        self.source.get(self.current).is_none()
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.current).cloned()
    }
    fn peek_next(&mut self, index: usize) -> Option<u8> {
        let target_idx = index + self.current;
        self.source.get(target_idx).cloned()
    }
    fn flush(&mut self) -> String {
        let lexeme = self.source.slice(self.start..self.current);
        self.source.advance(self.current);
        self.current = 0;
        self.start = 0;
        String::from_utf8(lexeme.to_vec()).unwrap()
    }
    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.flush();

        self.tokens.push(Token {
            t_type: token_type,
            lexeme,
            line: self.line,
        });
    }
    fn string(&mut self) -> Option<TokenType> {
        loop {
            let Some(b) = self.peek() else {
                unexpected_terminal_err(self.line);
                self.flush();
                return None;
            };
            if b == b'\n' {
                self.line += 1;
            }
            self.current += 1;
            if b == b'"' {
                let literal = self.source.slice(self.start + 1..self.current - 1);
                let literal = String::from_utf8(literal.to_vec()).unwrap();
                return Some(STRING(literal));
            }
        }
    }
    fn number(&mut self) -> MyResult<TokenType> {
        let mut met_dot = false;
        loop {
            let b = self.peek();
            let Some(b) = b else {
                break;
            };

            if b == b'.' {
                let is_next_digit = self
                    .peek_next(1)
                    .context("unexpected terminal")?
                    .is_ascii_digit();

                if is_next_digit {
                    self.current += 1;
                    met_dot = true;
                    continue;
                } else {
                    break;
                }
            }
            if !b.is_ascii_digit() {
                break;
            }
            self.current += 1;
        }
        let literal = self.source.slice(self.start..self.current);
        let literal = String::from_utf8(literal.to_vec()).unwrap();
        let n: f64 = literal.parse().unwrap_or_else(|_| panic!("parse f64 {:?}", literal));
        Ok(NUMBER(n))
    }
    fn identifier(&mut self) -> MyResult<TokenType> {
        loop {
            let b = self.peek();
            let Some(b) = b else {
                break;
            };
            if !(b.is_ascii_alphanumeric() || b == b'_') {
                break;
            }
            self.current += 1;
        }

        let ident = self.source.slice(self.start..self.current);
        let ident = String::from_utf8(ident.to_vec()).unwrap();
        let keyword_or_none = keywords_map.get(&ident);
        if let Some(keyword) = keyword_or_none {
            return Ok(keyword.clone());
        }
        Ok(IDENTIFIER(ident))
    }
}

#[test]
fn aa() {
    let src = Bytes::from(
        r#"
@
    "#,
    );
    let mut scanner = Scanner::new(src);
    scanner.scan_tokens().unwrap();
    scanner.print_tokens();
}
