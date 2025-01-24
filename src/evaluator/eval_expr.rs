use std::collections::HashMap;

use crate::{
    callable::Callable,
    data_types::scaler::{Scalar, UserFn},
    environment::{Environment, EnvironmentType},
    error::report_runtime,
    expr::{binary::BinaryExpr, Expr},
    stmt::Stmt,
    token::Token,
    token_type::TokenType,
    InterpretRet, InterpretRtErr,
};

use super::{error::InterpretResult, Evaluator, InterpretError, Interprete};

impl Interprete<Expr> for Evaluator {
    type Output = InterpretResult<Scalar>;
    fn eval(&mut self, expr: &Expr) -> InterpretResult<Scalar> {
        match expr {
            Expr::Binary(binary) => {
                let BinaryExpr {
                    left,
                    right,
                    operator,
                } = binary.as_ref();
                let left = self.eval(left)?;
                let right = self.eval(right)?;
                match operator.t_type {
                    TokenType::MINUS => {
                        self.check_number_operands(&left, &right, operator)?;
                        Ok(left - right)
                    }
                    TokenType::PLUS => {
                        if !matches!(
                            (&left, &right),
                            (Scalar::String(_), Scalar::String(_))
                                | (Scalar::Number(_), Scalar::Number(_))
                        ) {
                            report_runtime(
                                operator.line,
                                "Operands must be two numbers or two strings.".to_string(),
                            );
                            return InterpretRtErr!(;"bad eval");
                        }
                        Ok(left + right)
                    }
                    TokenType::SLASH => {
                        self.check_number_operands(&left, &right, operator)?;
                        Ok(left / right)
                    }
                    TokenType::STAR => {
                        self.check_number_operands(&left, &right, operator)?;
                        Ok(left * right)
                    }
                    TokenType::BangEqual => Ok(Scalar::Bool(left != right)),
                    TokenType::EqualEqual => Ok(Scalar::Bool(left == right)),
                    TokenType::GREATER => {
                        self.check_number_operands(&left, &right, operator)?;
                        Ok(Scalar::Bool(left > right))
                    }
                    TokenType::GreaterEqual => {
                        self.check_number_operands(&left, &right, operator)?;
                        Ok(Scalar::Bool(left >= right))
                    }
                    TokenType::LESS => {
                        self.check_number_operands(&left, &right, operator)?;
                        Ok(Scalar::Bool(left < right))
                    }
                    TokenType::LessEqual => {
                        self.check_number_operands(&left, &right, operator)?;
                        Ok(Scalar::Bool(left <= right))
                    }
                    _ => Ok(Scalar::Nil),
                }
            }
            Expr::Grouping(grouping) => self.eval(&grouping.expression),
            Expr::Literal(literal) => Ok(literal.value.clone()),
            Expr::Unary(unary) => match unary.operator.t_type {
                TokenType::BANG => Ok(!self.eval(&unary.right)?),
                TokenType::MINUS => {
                    let right = self.eval(&unary.right)?;
                    self.check_number_operand(&right, &unary.operator)?;
                    Ok(-right)
                }
                _ => Ok(Scalar::Nil),
            },
            Expr::Variable(variable) => {
                let distance = self.get_depth(expr);
                let value = if let Some(distance) = distance {
                    self.env.borrow().get_at(distance, &variable.name.lexeme)
                } else {
                    self.global.borrow().get(&variable.name.lexeme)
                };
                match value {
                    Ok(value) => Ok(value.clone()),
                    Err(_) => {
                        report_runtime(
                            variable.name.line,
                            format!("Access undefined variable '{}'.", variable.name.lexeme),
                        );
                        InterpretRtErr!(;"bad variable access")
                    }
                }
            }
            Expr::Assign(assign) => {
                let value = self.eval(&assign.value)?;
                let name = &assign.name.lexeme;
                let distance = self.get_depth(expr);
                let result = if let Some(distance) = distance {
                    self.env
                        .borrow_mut()
                        .assign_at(distance, name, value.clone())
                } else {
                    self.global.borrow_mut().assign(name, value.clone())
                };
                // let result = env
                //     .borrow_mut()
                //     .assign(name, value.clone());
                match result {
                    Ok(_) => Ok(value),
                    Err(_) => {
                        report_runtime(
                            assign.name.line,
                            format!("Assign to undefined variable '{}'.", assign.name.lexeme),
                        );
                        InterpretRtErr!(;"bad variable assign")
                    }
                }
            }
            Expr::Logical(logical) => {
                let left = self.eval(&logical.left)?;
                let left_condition = (!!left.clone()).as_bool().unwrap();
                match logical.operator.t_type {
                    TokenType::OR if left_condition => Ok(left),
                    TokenType::AND if !left_condition => Ok(left),
                    _ => self.eval(&logical.right),
                }
            }
            Expr::Call(call) => {
                let callee = self.eval(&call.callee)?;
                let mut args = Vec::new();
                for arg in &call.arguments {
                    args.push(self.eval(arg)?);
                }

                let function = callee.as_fun().ok_or_else(|| {
                    report_runtime(
                        call.parent.line,
                        "Can only call functions and classes.".to_string(),
                    );
                    InterpretError::rt("Can only call functions and classes.")
                })?;

                // Check if number of arguments matches
                if args.len() != function.arity() {
                    report_runtime(
                        call.parent.line,
                        format!(
                            "Expected {} arguments but got {}.",
                            function.arity(),
                            args.len()
                        ),
                    );
                    return InterpretRtErr!(;"wrong number of arguments");
                }

                function.call(self, args)
            }
        }
    }
}
