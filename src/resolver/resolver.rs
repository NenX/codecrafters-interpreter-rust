use std::collections::HashMap;

use crate::{
    error::my_error_token,
    evaluator::Evaluator,
    expr::Expr,
    stmt::Stmt,
    token::Token,
};

use super::ResolverWalk;

pub struct Resolver<'a> {
    scopes: Vec<HashMap<String, bool>>,
    evaluator: &'a mut Evaluator,
}

impl<'a> Resolver<'a> {
    pub fn new(evaluator: &'a mut Evaluator) -> Self {
        Self {
            scopes: vec![],
            evaluator,
        }
    }
    pub fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            self.resolve(stmt);
        }
    }
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn end_scope(&mut self) {
        self.scopes.pop();
    }
    fn declare(&mut self, token: &Token) {
        let name = token.lexeme.clone();
        let len = self.scopes.len();
        if len == 0 {
            return;
        }
        let scope = &mut self.scopes[len - 1];
        if scope.contains_key(&name) {
            my_error_token(
                token.clone(),
                "Already a variable with this name in this scope.".to_string(),
            );
        }
        scope.insert(name, false);
    }
    fn define(&mut self, token: &Token) {
        let name = token.lexeme.clone();
        let len = self.scopes.len();
        if len == 0 {
            return;
        }
        // 不需要检查是否已经存在，因为定义时，变量的声明不一定在当前作用域
        self.scopes[len - 1].insert(name, true);
    }
    fn resolve_local(&mut self, expr: &Expr, name: &str) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(name) {
                self.evaluator.resolve(expr, i);
                return;
            }
        }
    }
    fn cur_scope(&self) -> Option<&HashMap<String, bool>> {
        let len = self.scopes.len();
        if len == 0 {
            return None;
        }
        Some(&self.scopes[len - 1])
    }
}
impl ResolverWalk<Expr> for Resolver<'_> {
    fn resolve(&mut self, expr: &Expr) {
        match expr {
            Expr::Variable(variable_expr) => {
                let name = variable_expr.name.lexeme.clone();
                if self
                    .cur_scope()
                    .map_or(false, |scope| matches!(scope.get(&name), Some(false)))
                {
                    my_error_token(
                        variable_expr.name.clone(),
                        "Can't read local variable in its own initializer.".to_string(),
                    );
                }
                self.resolve_local(expr, &name);
            }
            Expr::Assign(assign_expr) => {
                self.resolve(&assign_expr.value);
                self.resolve_local(expr, &assign_expr.name.lexeme);
            }
            Expr::Binary(binary_expr) => {
                self.resolve(&binary_expr.left);
                self.resolve(&binary_expr.right);
            }
            Expr::Logical(logical_expr) => {
                self.resolve(&logical_expr.left);
                self.resolve(&logical_expr.right);
            }
            Expr::Grouping(grouping_expr) => {
                self.resolve(&grouping_expr.expression);
            }
            Expr::Literal(_) => {}
            Expr::Call(call_expr) => {
                self.resolve(&call_expr.callee);
                for arg in call_expr.arguments.iter() {
                    self.resolve(arg);
                }
            }
            Expr::Unary(unary_expr) => {
                self.resolve(&unary_expr.right);
            }
        }
    }
}

impl ResolverWalk<Stmt> for Resolver<'_> {
    fn resolve(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression(expression_stmt) => {
                self.resolve(&expression_stmt.expression);
            }
            Stmt::Print(print_stmt) => {
                self.resolve(&print_stmt.expression);
            }
            Stmt::Var(var_stmt) => {
                self.declare(&var_stmt.name);
                if let Some(initializer) = &var_stmt.initializer {
                    self.resolve(initializer);
                }
            }
            Stmt::Block(block_stmt) => {
                self.begin_scope();
                self.resolve_stmts(&block_stmt.statements);
                self.end_scope();
            }
            Stmt::If(if_stmt) => {
                self.resolve(&if_stmt.condition);
                self.resolve(&if_stmt.then_branch);
                if let Some(else_branch) = &if_stmt.else_branch {
                    self.resolve(else_branch);
                }
            }
            Stmt::While(while_stmt) => {
                self.resolve(&while_stmt.condition);
                self.resolve(&while_stmt.body);
            }
            Stmt::Function(function_stmt) => {
                self.declare(&function_stmt.name);
                self.define(&function_stmt.name);
                self.begin_scope();
                for param in function_stmt.params.iter() {
                    self.declare(param);
                    self.define(param);
                }
                self.resolve_stmts(&function_stmt.fn_body);
                self.end_scope();
            }
            Stmt::Return(return_stmt) => {
                if let Some(value) = &return_stmt.value {
                    self.resolve(value);
                }
            }
        }
    }
}
