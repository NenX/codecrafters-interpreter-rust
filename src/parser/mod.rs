pub mod parse_error;

use std::rc::Rc;

use crate::{
    error::{my_error_token, MyResult},
    expr::{
        assign::AssignExpr, binary::BinaryExpr, call::CallExpr, get::GetExpr,
        grouping::GroupingExpr, literal::LiteralExpr, logical::LogicalExpr, set::SetExpr,
        super_expr::SuperExpr, this::ThisExpr, unary::UnaryExpr, variable::VariableExpr, Expr,
    },
    stmt::{
        block::BlockStmt, class_stmt::ClassStmt, expression::ExpressionStmt,
        function::FunctionStmt, if_stmt::IfStmt, print::PrintStmt, return_stmt::ReturnStmt,
        var::VarStmt, while_stmt::WhileStmt, Stmt,
    },
    token::Token,
    token_type::{CmpTokenType, TokenType},
    MyErr,
};
use parse_error::ParseError;
use TokenType::*;

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
                None
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
        if self.match_advance_unchecked([CLASS]).is_some() {
            return self.class_declaration();
        };
        if self.match_advance_unchecked([FUN]).is_some() {
            return Ok(self.function_declaration()?.into());
        };
        if self.match_advance_unchecked([VAR]).is_some() {
            return self.var_declaration();
        };
        self.statement()
    }
    fn class_declaration(&mut self) -> MyResult<Stmt> {
        let name = self.consume(IDENTIFIER(String::new()), "")?;

        let mut superclass = None;

        if self.match_advance_unchecked([LESS]).is_some() {
            superclass = Some(self.consume(IDENTIFIER(String::new()), "Expect superclass name.")?);
        }
        self.consume(LeftBrace, "Expect '{' after class name.")?;
        let mut methods = vec![];
        while !self.check_unchecked([&RightBrace]) {
            methods.push(Rc::new(self.function_declaration()?));
        }
        self.consume(RightBrace, "Expect '}' after class body.")?;
        Ok(ClassStmt {
            name,
            methods,
            superclass: superclass.map(|name| VariableExpr { name }.into()),
        }
        .into())
    }

    fn function_declaration(&mut self) -> MyResult<FunctionStmt> {
        let name = self.consume(IDENTIFIER(String::new()), "")?;
        let _ = self.consume(LeftParen, "")?;
        let mut params = vec![];

        if !self.check_unchecked([&RightParen]) {
            params.push(self.consume(IDENTIFIER(String::new()), "Expect parameter name.")?);

            while self.match_advance_unchecked([COMMA]).is_some() {
                params.push(self.consume(IDENTIFIER(String::new()), "Expect parameter name.")?);
            }
        }

        let _ = self.consume(RightParen, "Expect ')' after parameters.")?;
        let _ = self.consume(LeftBrace, r"Expect '{' after parameters.")?;

        let fn_body = self.block_stmt()?;

        Ok(FunctionStmt {
            name,
            params,
            fn_body,
        })
    }
    fn var_declaration(&mut self) -> MyResult<Stmt> {
        let name = self.consume(IDENTIFIER(String::new()), "")?;
        let mut initializer = None;
        if self.match_advance_unchecked([EQUAL]).is_some() {
            let next = self.expression()?;

            initializer = Some(next)
        }
        self.consume(SEMICOLON, "Var Stmt Expect '}' after block.")?;

        Ok(VarStmt { name, initializer }.into())
    }
    fn statement(&mut self) -> MyResult<Stmt> {
        if self.match_advance_unchecked([RETURN]).is_some() {
            return self.return_stmt();
        }
        if self.match_advance_unchecked([FOR]).is_some() {
            return self.for_stmt();
        }
        if self.match_advance_unchecked([IF]).is_some() {
            return self.if_stmt();
        }
        if self.match_advance_unchecked([PRINT]).is_some() {
            return self.print_stmt();
        }
        if self.match_advance_unchecked([WHILE]).is_some() {
            return self.while_stmt();
        }
        if self.match_advance_unchecked([LeftBrace]).is_some() {
            return self
                .block_stmt()
                .map(|v| BlockStmt { statements: v }.into());
        }

        self.expression_stmt()
    }
    fn if_stmt(&mut self) -> MyResult<Stmt> {
        self.consume(LeftParen, "Expect '(' after 'if'.")?;

        let condition = self.expression()?;

        self.consume(RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;

        if self.match_advance_unchecked([ELSE]).is_some() {
            Ok(IfStmt {
                then_branch,
                else_branch: Some(self.statement()?),
                condition,
            }
            .into())
        } else {
            Ok(IfStmt {
                then_branch,
                else_branch: None,
                condition,
            }
            .into())
        }
    }
    fn while_stmt(&mut self) -> MyResult<Stmt> {
        self.consume(LeftParen, "Expect '(' after 'while'.")?;

        let condition = self.expression()?;
        self.consume(RightParen, "Expect ')' after while condition.")?;

        let body = self.statement()?;

        Ok(WhileStmt { condition, body }.into())
    }
    fn return_stmt(&mut self) -> MyResult<Stmt> {
        let keyword = self.previous_unchecked();
        let mut value = None;
        if !self.check_unchecked([&SEMICOLON]) {
            value = self.expression()?.into();
        }
        self.consume(SEMICOLON, "Expect ';' after return value.")?;
        Ok(ReturnStmt { keyword, value }.into())
    }
    fn for_stmt(&mut self) -> MyResult<Stmt> {
        self.consume(LeftParen, "Expect '(' after 'loop'.")?;
        let mut body = BlockStmt::from([]);
        let mut initial = None;

        if self.match_advance_unchecked([SEMICOLON]).is_none() {
            if self.match_advance_unchecked([VAR]).is_some() {
                initial = Some(self.var_declaration()?)
            } else {
                initial = Some(self.expression_stmt()?);
            }
        }

        let second = self.peek_unchecked().t_type;

        let condition;
        if second != SEMICOLON {
            condition = self.expression()?;
        } else {
            condition = LiteralExpr::from(true).into();
        }
        self.consume(SEMICOLON, "Expect ';' after loop condition.")?;

        let third = self.peek_unchecked().t_type;

        let mut increment = None;
        if third != RightParen {
            increment = Some(self.expression()?);
        }

        self.consume(RightParen, "Expect ')' after loop condition.")?;

        body.push(self.statement()?);

        if let Some(incremnt) = increment {
            body.push(ExpressionStmt::from(incremnt).into());
        }

        let mut while_or_block: Stmt = WhileStmt {
            condition,
            body: body.into(),
        }
        .into();
        // println!("init {:#?}", initial);

        if let Some(init) = initial {
            let bb = [init, while_or_block];
            // println!("for {:#?}", bb);

            while_or_block = BlockStmt::from(bb).into();
        }
        Ok(while_or_block)
    }
    // 为了给函数使用，返回一个Vec<Stmt>，而不是Stmt
    fn block_stmt(&mut self) -> MyResult<Vec<Stmt>> {
        let mut statements = vec![];
        loop {
            if self.check_unchecked([&EOF, &RightBrace]) {
                break;
            }
            let stmt = self.declaration()?;
            statements.push(stmt);
        }
        self.consume(RightBrace, "Block Stmt Expect '}' after block.")?;
        // Ok(BlockStmt { statements }.into())
        Ok(statements)
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
        self.assignment()
    }
    fn assignment(&mut self) -> MyResult<Expr> {
        let expr = self.or()?;
        if let Some(equal) = self.match_advance_unchecked([EQUAL]) {
            let value = self.assignment()?;
            match expr {
                Expr::Variable(variable_expr) => {
                    return Ok(AssignExpr {
                        name: variable_expr.name,
                        value,
                    }
                    .into())
                }
                Expr::Get(get_expr) => {
                    return Ok(SetExpr {
                        object: get_expr.object,
                        name: get_expr.name,
                        value,
                    }
                    .into())
                }
                _ => {
                    return MyErr!(,ParseError::NotExpected(equal, "Invalid assignment target.".to_string()))
                }
            }
        }

        Ok(expr)
    }
    fn or(&mut self) -> MyResult<Expr> {
        let mut expr = self.and()?;
        while let Some(x) = self.match_advance_unchecked([OR]) {
            expr = LogicalExpr {
                left: expr,
                right: self.and()?,
                operator: x,
            }
            .into();
        }
        Ok(expr)
    }
    fn and(&mut self) -> MyResult<Expr> {
        let mut expr = self.equality()?;

        while let Some(x) = self.match_advance_unchecked([AND]) {
            expr = LogicalExpr {
                left: expr,
                right: self.equality()?,
                operator: x,
            }
            .into();
        }

        Ok(expr)
    }
    fn equality(&mut self) -> MyResult<Expr> {
        let mut expr = self.comparision()?;

        while let Some(operator) = self.match_advance_unchecked([EqualEqual, BangEqual]) {
            expr = BinaryExpr {
                left: expr,
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
                left: expr,
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
                left: expr,
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
                left: expr,
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
        self.call()
    }
    fn call(&mut self) -> MyResult<Expr> {
        let mut expr = self.primary()?;

        // while self.match_advance_unchecked([LeftParen]).is_some() {
        //     expr = self.finish_call(expr)?;
        // }
        loop {
            let next = self.peek_unchecked();
            if next.t_type == LeftParen {
                self.current += 1;
                expr = self.finish_call(expr)?;
            } else if next.t_type == DOT {
                self.current += 1;
                expr = GetExpr {
                    object: expr,
                    name: self
                        .consume(IDENTIFIER(String::new()), "Expect property name after '.'.")?,
                }
                .into();
            } else {
                break;
            }
        }
        Ok(expr)
    }
    fn finish_call(&mut self, callee: Expr) -> MyResult<Expr> {
        let mut arguments = vec![];
        let is_right_paren = self.check_unchecked([&RightParen]);

        if !is_right_paren {
            arguments.push(self.expression()?);
            while self.match_advance_unchecked([COMMA]).is_some() {
                if arguments.len() >= 255 {
                    return MyErr!(,ParseError::NotExpected(self.peek_unchecked(), "Can't have more than 255 arguments.".to_string()));
                }
                arguments.push(self.expression()?);
            }
        }

        let parent = self.consume(RightParen, "message")?;
        Ok(CallExpr {
            callee,
            arguments,
            parent,
        }
        .into())
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
            THIS => ThisExpr { keyword: next }.into(),
            SUPER => {
                self.consume(DOT, "Expect '.' after 'super'.")?;
                let method =
                    self.consume(IDENTIFIER(String::new()), "Expect superclass method name.")?;
                SuperExpr {
                    keyword: next,
                    method,
                }
                .into()
            }
            _ => {
                return MyErr!(,ParseError::NotExpected(next, "[Parser] Expect expression.".to_string()))
            }
        };
        Ok(expr)
    }
    fn is_at_end(&self) -> bool {
        if self.current >= self.end {
            return true;
        }
        self.peek_unchecked().t_type == EOF
    }
    fn previous_unchecked(&self) -> Token {
        self.tokens
            .get(self.current - 1)
            .expect("previous token")
            .clone()
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
    fn consume(&mut self, token: TokenType, msg: impl AsRef<str>) -> MyResult<Token> {
        if self.check_unchecked([&token]) {
            return Ok(self.advance_unchecked());
        };
        eprintln!(
            "expect {:?} but received {:?}",
            &token,
            self.peek_unchecked()
        );
        MyErr!(,ParseError::NotExpected(self.peek_unchecked(), msg .as_ref(). to_string()))
    }
}
