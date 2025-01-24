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

use super::{error::InterpretResult, AstInterpreter, InterpretError};

pub struct Evaluator {
    locals: HashMap<*const Expr, usize>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
        }
    }

    fn check_number_operands(
        &self,
        left: &Scalar,
        right: &Scalar,
        operator: &Token,
    ) -> InterpretResult<()> {
        if Scalar::check_number_operands(left, right) {
            Ok(())
        } else {
            report_runtime(operator.line, format!("Operands must be numbers."));
            InterpretRtErr!(;"bad eval")
        }
    }

    fn check_number_operand(&self, right: &Scalar, operator: &Token) -> InterpretResult<()> {
        if matches!(right, Scalar::Number(_)) {
            Ok(())
        } else {
            report_runtime(operator.line, format!("Operand must be a number."));
            InterpretRtErr!(;"bad eval")
        }
    }
}
impl AstInterpreter<Expr> for Evaluator {
    type Output = InterpretResult<Scalar>;
    fn eval(&mut self, expr: &Expr, env: EnvironmentType) -> InterpretResult<Scalar> {
        match expr {
            Expr::Binary(binary) => {
                let BinaryExpr {
                    left,
                    right,
                    operator,
                } = binary.as_ref();
                let left = self.eval(left, env.clone())?;
                let right = self.eval(right, env)?;
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
            Expr::Grouping(grouping) => self.eval(&grouping.expression, env),
            Expr::Literal(literal) => Ok(literal.value.clone()),
            Expr::Unary(unary) => match unary.operator.t_type {
                TokenType::BANG => Ok(!self.eval(&unary.right, env.clone())?),
                TokenType::MINUS => {
                    let right = self.eval(&unary.right, env.clone())?;
                    self.check_number_operand(&right, &unary.operator)?;
                    Ok(-right)
                }
                _ => Ok(Scalar::Nil),
            },
            Expr::Variable(variable) => {
                let value = env.borrow().get(&variable.name.lexeme);
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
                let value = self.eval(&assign.value, env.clone())?;
                let result = env
                    .borrow_mut()
                    .assign(assign.name.lexeme.clone(), value.clone());
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
                let left = self.eval(&logical.left, env.clone())?;
                let left_condition = (!!left.clone()).as_bool().unwrap();
                match logical.operator.t_type {
                    TokenType::OR if left_condition => Ok(left),
                    TokenType::AND if !left_condition => Ok(left),
                    _ => self.eval(&logical.right, env),
                }
            }
            Expr::Call(call) => {
                let callee = self.eval(&call.callee, env.clone())?;
                let mut args = Vec::new();
                for arg in &call.arguments {
                    args.push(self.eval(arg, env.clone())?);
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
impl AstInterpreter<Stmt> for Evaluator {
    type Output = InterpretResult<()>;

    fn eval(&mut self, stmt: &Stmt, env: EnvironmentType) -> InterpretResult<()> {
        match stmt {
            Stmt::Var(var) => {
                let value = match &var.initializer {
                    Some(expr) => Some(self.eval(expr, env.clone())?),
                    None => None,
                };
                env.borrow_mut().define(var.name.lexeme.clone(), value);
                Ok(())
            }
            Stmt::Expression(expr) => {
                self.eval(&expr.expression, env)?;
                Ok(())
            }
            Stmt::Block(block) => {
                let new_env = Environment::new(Some(env), Some("block"));
                for stmt in &block.statements {
                    self.eval(stmt, new_env.clone())?;
                }
                Ok(())
            }
            Stmt::Print(print) => {
                let value = self.eval(&print.expression, env)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::If(if_stmt) => {
                let condition = self.eval(&if_stmt.condition, env.clone())?;
                if (!!condition).as_bool().unwrap() {
                    self.eval(&if_stmt.then_branch, env)
                } else if let Some(else_branch) = &if_stmt.else_branch {
                    self.eval(else_branch, env)
                } else {
                    Ok(())
                }
            }
            Stmt::While(while_stmt) => {
                while let Some(condition) = self.eval(&while_stmt.condition, env.clone())?.as_bool()
                {
                    if !condition {
                        break;
                    }
                    self.eval(&while_stmt.body, env.clone())?;
                }
                Ok(())
            }
            Stmt::Function(func) => {
                let fun = UserFn::new(env.clone(), func.as_ref().clone());
                env.borrow_mut()
                    .define(func.name.lexeme.clone(), Some(fun.into()));
                Ok(())
            }
            Stmt::Return(ret) => {
                let value = match &ret.value {
                    Some(expr) => self.eval(expr, env)?,
                    None => Scalar::Nil,
                };
                InterpretRet!(value)
            }
        }
    }
}
