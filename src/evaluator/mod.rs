use crate::{
    ast_interpreter::interpret_err::{InterpretError, InterpretResult},
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

pub struct Evaluator;

impl Evaluator {
    pub fn interpret_expr(expr: &Expr, env: EnvironmentType) -> InterpretResult<Scalar> {
        match expr {
            Expr::Binary(binary) => {
                let BinaryExpr {
                    left,
                    right,
                    operator,
                } = binary.as_ref();
                let left = Self::interpret_expr(left, env.clone())?;
                let right = Self::interpret_expr(right, env)?;
                match operator.t_type {
                    TokenType::MINUS => {
                        Self::check_number_operands(&left, &right, operator)?;
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
                        Self::check_number_operands(&left, &right, operator)?;
                        Ok(left / right)
                    }
                    TokenType::STAR => {
                        Self::check_number_operands(&left, &right, operator)?;
                        Ok(left * right)
                    }
                    TokenType::BangEqual => Ok(Scalar::Bool(left != right)),
                    TokenType::EqualEqual => Ok(Scalar::Bool(left == right)),
                    TokenType::GREATER => {
                        Self::check_number_operands(&left, &right, operator)?;
                        Ok(Scalar::Bool(left > right))
                    }
                    TokenType::GreaterEqual => {
                        Self::check_number_operands(&left, &right, operator)?;
                        Ok(Scalar::Bool(left >= right))
                    }
                    TokenType::LESS => {
                        Self::check_number_operands(&left, &right, operator)?;
                        Ok(Scalar::Bool(left < right))
                    }
                    TokenType::LessEqual => {
                        Self::check_number_operands(&left, &right, operator)?;
                        Ok(Scalar::Bool(left <= right))
                    }
                    _ => Ok(Scalar::Nil),
                }
            }
            Expr::Grouping(grouping) => Self::interpret_expr(&grouping.expression, env),
            Expr::Literal(literal) => Ok(literal.value.clone()),
            Expr::Unary(unary) => match unary.operator.t_type {
                TokenType::BANG => Ok(!Self::interpret_expr(&unary.right, env.clone())?),
                TokenType::MINUS => {
                    let right = Self::interpret_expr(&unary.right, env.clone())?;
                    Self::check_number_operand(&right, &unary.operator)?;
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
                let value = Self::interpret_expr(&assign.value, env.clone())?;
                let result = env.borrow_mut().assign(assign.name.lexeme.clone(), value.clone());
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
                let left = Self::interpret_expr(&logical.left, env.clone())?;
                let left_condition = (!!left.clone()).as_bool().unwrap();
                match logical.operator.t_type {
                    TokenType::OR if left_condition => Ok(left),
                    TokenType::AND if !left_condition => Ok(left),
                    _ => Self::interpret_expr(&logical.right, env),
                }
            }
            Expr::Call(call) => {
                let callee = Self::interpret_expr(&call.callee, env.clone())?;
                let mut args = Vec::new();
                for arg in &call.arguments {
                    args.push(Self::interpret_expr(arg, env.clone())?);
                }

                let function = callee
                    .as_fun()
                    .ok_or_else(|| {
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

                function.call(args)
            }
        }
    }

    pub fn interpret_stmt(stmt: &Stmt, env: EnvironmentType) -> InterpretResult<()> {
        match stmt {
            Stmt::Var(var) => {
                let value = match &var.initializer {
                    Some(expr) => Some(Self::interpret_expr(expr, env.clone())?),
                    None => None,
                };
                env.borrow_mut().define(var.name.lexeme.clone(), value);
                Ok(())
            }
            Stmt::Expression(expr) => {
                Self::interpret_expr(&expr.expression, env)?;
                Ok(())
            }
            Stmt::Block(block) => {
                let new_env = Environment::new(Some(env), Some("block"));
                for stmt in &block.statements {
                    Self::interpret_stmt(stmt, new_env.clone())?;
                }
                Ok(())
            }
            Stmt::Print(print) => {
                let value = Self::interpret_expr(&print.expression, env)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::If(if_stmt) => {
                let condition = Self::interpret_expr(&if_stmt.condition, env.clone())?;
                if (!!condition).as_bool().unwrap() {
                    Self::interpret_stmt(&if_stmt.then_branch, env)
                } else if let Some(else_branch) = &if_stmt.else_branch {
                    Self::interpret_stmt(else_branch, env)
                } else {
                    Ok(())
                }
            }
            Stmt::While(while_stmt) => {
                while let Some(condition) =
                    Self::interpret_expr(&while_stmt.condition, env.clone())?.as_bool()
                {
                    if !condition {
                        break;
                    }
                    Self::interpret_stmt(&while_stmt.body, env.clone())?;
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
                    Some(expr) => Self::interpret_expr(expr, env)?,
                    None => Scalar::Nil,
                };
                InterpretRet!(value)
            }
        }
    }

    fn check_number_operands(
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

    fn check_number_operand(right: &Scalar, operator: &Token) -> InterpretResult<()> {
        if matches!(right, Scalar::Number(_)) {
            Ok(())
        } else {
            report_runtime(operator.line, format!("Operand must be a number."));
            InterpretRtErr!(;"bad eval")
        }
    }
}
