pub mod parse_error;

use crate::{
    error::{my_error_token, MyError, MyResult},
    expr::{self, binary::Binary, grouping::Grouping, literal::Literal, unary::Unary, Expr},
    token::Token,
    token_type::TokenType,
    MyErr,
};
use parse_error::ParseError;
use TokenType::*;
/*
expression -> equality
equality -> comparision ( ( "!=" | "==" ) comparision )*
comparision -> term ( ( ">" | ">=" | "<" | "<=" ) term )*
term -> factor ( ( "+" | "-" ) factor )*
factor -> unary ( ( "*" | "/" ) unary )*
unary -> ( "!" | "-" )* unary | primary
primary -> STRING | NUMBER | "false" | "true" | "nil" | "(" expression ")"
*/
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    end: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            end: tokens.len(),
            tokens,
            current: 0,
        }
    }
    pub fn parse(&mut self) -> Option<Expr> {
        let res = self.expression();
        match res {
            Ok(expr) => Some(expr),
            Err(e) => {
                if let Ok(e) = e.downcast::<ParseError>() {
                    match e {
                        ParseError::NotExpected(token, message) => {
                            my_error_token(token, message);
                        }
                    }
                }
                return None;
            }
        }
    }
    fn expression(&mut self) -> MyResult<Expr> {
        self.equality()
    }
    fn equality(&mut self) -> MyResult<Expr> {
        let mut expr = self.comparision()?;

        while let Some(operator) = self.match_advance_unchecked([EqualEqual, BangEqual]) {
            expr = Binary {
                letf: expr,
                operator,
                right: self.comparision()?,
            }
            .into();
        }

        Ok(expr)
    }
    fn comparision(&mut self) -> MyResult<Expr> {
        let mut expr = self.term()?;

        while let Some(operator) =
            self.match_advance_unchecked([GREATER, GreaterEqual, LESS, LessEqual])
        {
            expr = Binary {
                letf: expr,
                operator,
                right: self.term()?,
            }
            .into();
        }

        Ok(expr)
    }
    fn term(&mut self) -> MyResult<Expr> {
        let mut expr = self.factor()?;

        while let Some(operator) = self.match_advance_unchecked([PLUS, MINUS]) {
            expr = Binary {
                letf: expr,
                operator,
                right: self.factor()?,
            }
            .into();
        }

        Ok(expr)
    }
    fn factor(&mut self) -> MyResult<Expr> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.match_advance_unchecked([SLASH, STAR]) {
            expr = Binary {
                letf: expr,
                operator,
                right: self.unary()?,
            }
            .into();
        }

        Ok(expr)
    }

    fn unary(&mut self) -> MyResult<Expr> {
        if let Some(operator) = self.match_advance_unchecked([BANG, MINUS]) {
            return Ok(Unary {
                operator,
                right: self.unary()?,
            }
            .into());
        }
        self.primary()
    }
    fn primary(&mut self) -> MyResult<Expr> {
        let next = self.advance_unchecked();
        let expr = match next.token_type {
            FALSE => Literal::from(false).into(),
            TRUE => Literal::from(true).into(),
            NIL => Literal::nil().into(),
            STRING(s) => Literal::from(s.as_str()).into(),
            NUMBER(i) => Literal::from(i).into(),
            LeftParen => {
                let expr = self.expression()?;
                self.consume(RightParen, "Expect ')' after expression.")?;
                Grouping::from(expr).into()
            }
            // IDENTIFIER(_) => todo!(),
            _ => return MyErr!(,ParseError::NotExpected(next, format!("Expect expression."))),
        };
        Ok(expr)
    }
    pub fn is_at_end(&self) -> bool {
        if self.current >= self.end {
            return true;
        }
        self.peek_unchecked().token_type == EOF
    }
    pub fn peek_unchecked(&self) -> Token {
        self.tokens.get(self.current).expect("peek token").clone()
    }
    pub fn advance_unchecked(&mut self) -> Token {
        let next = self.peek_unchecked();
        self.current += 1;
        next
    }
    pub fn check_unchecked(&self, targets: impl IntoIterator<Item = TokenType>) -> bool {
        let next = &self.peek_unchecked();
        targets.into_iter().any(|t| next.token_type == t)
    }
    pub fn match_advance_unchecked(
        &mut self,
        targets: impl IntoIterator<Item = TokenType>,
    ) -> Option<Token> {
        let next = self.peek_unchecked();
        let is_match = targets.into_iter().any(|t| next.token_type == t);
        if is_match {
            self.current += 1;
            Some(next)
        } else {
            None
        }
    }
    fn consume(&mut self, token: TokenType, message: &str) -> MyResult<Token> {
        if self.check_unchecked([token]) {
            return Ok(self.advance_unchecked());
        };

        MyErr!(,ParseError::NotExpected(self.peek_unchecked(), message.to_string()))
    }
}
