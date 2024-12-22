pub mod parse_error;

use crate::{
    error::{my_error_token, MyError, MyResult},
    expr::{
        self, binary::BinaryExpr, grouping::GroupingExpr, literal::LiteralExpr, unary::UnaryExpr,
        variable::VariableExpr, Expr,
    },
    stmt::{block::BlockStmt, expression::ExpressionStmt, print::PrintStmt, var::VarStmt, Stmt},
    token::Token,
    token_type::{CmpTokenType, TokenType},
    MyErr,
};
use parse_error::ParseError;
use TokenType::*;
/*
program        → declaration* EOF
declaration    → varDecl | statement
varDecl        → "var" IDENTIFIER ( "=" expression )? ";"
statement      → exprStmt | printStmt | block
block          → "{" declaration* "}"
exprStmt       → expression ";"
printStmt      → "print" expression ";"
expression     → assignment
assignment     → IDENTIFIER "=" assignment | equality ;
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
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut v = vec![];
        while !self.is_at_end() {
            let stmt = self.declaration_checked();
            if let Some(stmt) = stmt {
                v.push(stmt);
            }
        }
        v
    }
    pub fn parse_expression(&mut self) -> Option<Expr> {
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
    fn synchronize(&mut self) {
        self.advance_unchecked();

        while !self.is_at_end() {
            let next = self.peek_unchecked();
            if [WHILE, CLASS, FUN, FOR, IF, PRINT, RETURN, VAR].contains(&next.t_type) {
                return;
            }
            if [SEMICOLON].contains(&next.t_type) {
                self.advance_unchecked();
                return;
            }
            self.advance_unchecked();
        }
    }
    fn declaration_checked(&mut self) -> Option<Stmt> {
        match self.declaration() {
            Ok(stmt) => Some(stmt),
            Err(e) => {
                if let Ok(e) = e.downcast::<ParseError>() {
                    match e {
                        ParseError::NotExpected(token, message) => {
                            my_error_token(token, message);
                        }
                    }
                }
                self.synchronize();
                None
            }
        }
    }
    fn declaration(&mut self) -> MyResult<Stmt> {
        if self.match_advance_unchecked([VAR]).is_some() {
            return self.var_declaration();
        };
        self.statement()
    }
    fn var_declaration(&mut self) -> MyResult<Stmt> {
        let name = self.consume(IDENTIFIER(format!("")), "")?;
        let mut initializer = None;
        if let Some(_) = self.match_advance_unchecked([EQUAL]) {
            let next = self.expression()?;

            initializer = Some(next)
        }
        self.consume(SEMICOLON, "Var Stmt Expect '}' after block.")?;

        Ok(VarStmt { name, initializer }.into())
    }
    fn statement(&mut self) -> MyResult<Stmt> {
        if self.match_advance_unchecked([PRINT]).is_some() {
            return self.print_stmt();
        }
        if self.match_advance_unchecked([LeftBrace]).is_some() {
            return self.block_stmt();
        }
        self.expression_stmt()
    }
    fn block_stmt(&mut self) -> MyResult<Stmt> {
        let mut statements = vec![];
        loop {
            if self.check_unchecked([&EOF, &RightBrace]) {
                break;
            }
            let stmt = self.declaration()?;
            statements.push(stmt);
        }
        self.consume(RightBrace, "Block Stmt Expect '}' after block.")?;
        Ok(BlockStmt { statements }.into())
    }
    fn print_stmt(&mut self) -> MyResult<Stmt> {
        let expression = self.expression()?;
        self.consume(SEMICOLON, "Print Stmt Expect ';' after expression.")?;
        Ok(PrintStmt { expression }.into())
    }
    fn expression_stmt(&mut self) -> MyResult<Stmt> {
        let expression = self.expression()?;
        self.consume(SEMICOLON, "Expression Stmt Expect ';' after expression.")?;
        Ok(ExpressionStmt { expression }.into())
    }
    fn expression(&mut self) -> MyResult<Expr> {
        self.equality()
    }
    fn equality(&mut self) -> MyResult<Expr> {
        let mut expr = self.comparision()?;

        while let Some(operator) = self.match_advance_unchecked([EqualEqual, BangEqual]) {
            expr = BinaryExpr {
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
            expr = BinaryExpr {
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
            expr = BinaryExpr {
                letf: expr,
                operator,
                right: self.factor()?,
            }
            .into();
            // println!("expr {:?}", expr)
        }

        Ok(expr)
    }
    fn factor(&mut self) -> MyResult<Expr> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.match_advance_unchecked([SLASH, STAR]) {
            expr = BinaryExpr {
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
            return Ok(UnaryExpr {
                operator,
                right: self.unary()?,
            }
            .into());
        }
        self.primary()
    }
    fn primary(&mut self) -> MyResult<Expr> {
        let next = self.advance_unchecked();
        let expr = match next.t_type {
            FALSE => LiteralExpr::from(false).into(),
            TRUE => LiteralExpr::from(true).into(),
            NIL => LiteralExpr::nil().into(),
            STRING(s) => LiteralExpr::from(s.as_str()).into(),
            NUMBER(i) => LiteralExpr::from(i).into(),
            LeftParen => {
                let expr = self.expression()?;
                self.consume(RightParen, "Expect ')' after expression.")?;
                GroupingExpr::from(expr).into()
            }
            IDENTIFIER(_) => VariableExpr { name: next }.into(),
            _ => return MyErr!(,ParseError::NotExpected(next, format!("Expect expression."))),
        };
        Ok(expr)
    }
    fn is_at_end(&self) -> bool {
        if self.current >= self.end {
            return true;
        }
        self.peek_unchecked().t_type == EOF
    }
    fn peek_unchecked(&self) -> Token {
        self.tokens.get(self.current).expect("peek token").clone()
    }
    fn advance_unchecked(&mut self) -> Token {
        let next = self.peek_unchecked();
        self.current += 1;
        next
    }
    fn check_unchecked<'a>(&self, targets: impl IntoIterator<Item = &'a TokenType>) -> bool {
        let next = &self.peek_unchecked();
        targets.into_iter().any(|t| next.is_same_type(t))
    }
    fn match_advance_unchecked(
        &mut self,
        targets: impl IntoIterator<Item = TokenType>,
    ) -> Option<Token> {
        let next = self.peek_unchecked();
        let is_match = targets.into_iter().any(|t| next.is_same_type(&t));
        if is_match {
            self.current += 1;
            Some(next)
        } else {
            None
        }
    }
    fn consume(&mut self, token: TokenType, message: &str) -> MyResult<Token> {
        if self.check_unchecked([&token]) {
            return Ok(self.advance_unchecked());
        };
        eprintln!(
            "expect {:?} but received {:?}",
            &token,
            self.peek_unchecked()
        );
        MyErr!(,ParseError::NotExpected(self.peek_unchecked(), message.to_string()))
    }
}
