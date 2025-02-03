use std::collections::HashMap;

use crate::{error::my_error_token, evaluator::Evaluator, expr::Expr, stmt::Stmt, token::Token};

use super::{ClassType, FunctionType, Resolver, ResolverWalk};

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
            Expr::Get(get_expr) => {
                self.resolve(&get_expr.object);
            }
            Expr::Set(set_expr) => {
                self.resolve(&set_expr.value);
                self.resolve(&set_expr.object);
            }
            Expr::This(this_expr) => {}
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
                self.resolve_function(function_stmt, FunctionType::Function);
                // self.begin_scope();
                // for param in function_stmt.params.iter() {
                //     self.declare(param);
                //     self.define(param);
                // }
                // self.resolve_stmts(&function_stmt.fn_body);
                // self.end_scope();
            }
            Stmt::Return(return_stmt) => {
                if !self.is_function() {
                    my_error_token(
                        return_stmt.keyword.clone(),
                        "Can't return from top-level code.".to_string(),
                    );
                }
                if let Some(value) = &return_stmt.value {
                    if self.is_initializer() {
                        my_error_token(
                            return_stmt.keyword.clone(),
                            "Can't return a value from an initializer.".to_string(),
                        );
                    }
                    self.resolve(value);
                }
            }
            Stmt::Class(class_stmt) => {
                let enclosing_class = self.class_type;
                self.set_class_type(ClassType::Class);
                self.declare(&class_stmt.name);
                self.define(&class_stmt.name);

                self.begin_scope();
                self.cur_scope().unwrap().insert("this".to_string(), true);
                for function in class_stmt.methods.iter() {
                    let function_type = if function.name.lexeme == "init" {
                        FunctionType::Method
                    } else {
                        FunctionType::Initializer
                    };
                    self.resolve_function(function.as_ref(), function_type);
                }
                self.end_scope();
                self.set_class_type(enclosing_class);
            }
        }
    }
}
