use crate::{
    callable::Callable,
    data_types::scaler::Scalar,
    environment::{Environment, EnvironmentType},
    error::report_runtime,
    expr::{binary::BinaryExpr, Expr},
    token::Token,
    token_type::TokenType,
    InterpretRtErr,
};

use super::{
    interpret_err::{InterpretError, InterpretResult},
    AstInterpreter,
};

impl AstInterpreter for Expr {
    type Output = InterpretResult<Scalar>;

    fn interpret(&self, env: EnvironmentType) -> Self::Output {
        self.interpret_checked(env)
    }
    // fn interpret(&self, env: &mut Environment) -> Self::Output {
    //     let scaler = match self.interpret_checked(env) {
    //         Ok(sc) => {
    //             sc
    //         }
    //         Err(e) => Scalar::Nil,
    //     };
    //     scaler
    // }
}
impl Expr {
    fn interpret_checked(&self, env: EnvironmentType) -> InterpretResult<Scalar> {
        let value = match self {
            Expr::Binary(binary) => {
                let BinaryExpr {
                    letf,
                    right,
                    operator,
                } = binary.as_ref();
                let left = letf.interpret_checked(env.clone())?;
                let right = right.interpret_checked(env)?;
                match operator.t_type {
                    TokenType::MINUS => {
                        check_number_operands(&left, &right, &operator)?;
                        left - right
                    }
                    TokenType::PLUS => {
                        if !matches!(
                            (&left, &right),
                            (Scalar::String(_), Scalar::String(_))
                                | (Scalar::Number(_), Scalar::Number(_))
                        ) {
                            report_runtime(
                                operator.line,
                                format!("Operands must be two numbers or two strings."),
                            );

                            return InterpretRtErr!(;"bad eval");
                        };
                        left + right
                    }
                    TokenType::SLASH => {
                        check_number_operands(&left, &right, &operator)?;
                        left / right
                    }
                    TokenType::STAR => {
                        check_number_operands(&left, &right, &operator)?;
                        left * right
                    }

                    TokenType::BangEqual => {
                        // check_number_operands(&left, &right, operator)?;
                        Scalar::Bool(left != right)
                    }
                    TokenType::EqualEqual => {
                        // check_number_operands(&left, &right, operator)?;
                        Scalar::Bool(left == right)
                    }
                    TokenType::GREATER => {
                        check_number_operands(&left, &right, operator)?;
                        Scalar::Bool(left > right)
                    }
                    TokenType::GreaterEqual => {
                        check_number_operands(&left, &right, operator)?;
                        Scalar::Bool(left >= right)
                    }
                    TokenType::LESS => {
                        check_number_operands(&left, &right, operator)?;
                        Scalar::Bool(left < right)
                    }
                    TokenType::LessEqual => {
                        check_number_operands(&left, &right, operator)?;
                        Scalar::Bool(left <= right)
                    }
                    _ => Scalar::Nil,
                }
            }
            Expr::Grouping(grouping) => grouping.expression.interpret_checked(env)?,
            Expr::Literal(literal) => literal.value.clone(),
            Expr::Unary(unary) => match unary.operator.t_type {
                TokenType::BANG => !unary.right.interpret_checked(env.clone())?,
                TokenType::MINUS => {
                    let right = unary.right.interpret_checked(env.clone())?;
                    check_number_operand(&right, &unary.operator)?;
                    -unary.right.interpret_checked(env)?
                }
                _ => Scalar::Nil,
            },
            Expr::Variable(variable) => {
                let value = env.borrow().get(&variable.name.lexeme);
                match value {
                    Ok(value) => value.clone(),
                    Err(_) => {
                        report_runtime(
                            variable.name.line,
                            format!("Access undefined variable '{}'.", variable.name.lexeme),
                        );
                        return InterpretRtErr!(;"bad variable access");
                    }
                }
            }
            Expr::Assign(assign) => {
                let v = assign.value.interpret_checked(env.clone())?;
                let value = env
                    .borrow_mut()
                    .assign(assign.name.lexeme.clone(), v.clone());
                match value {
                    Ok(_) => v,
                    Err(_) => {
                        report_runtime(
                            assign.name.line,
                            format!("Assign to undefined variable '{}'.", assign.name.lexeme),
                        );
                        return InterpretRtErr!(;"bad variable assign");
                    }
                }
            }
            Expr::Logical(logical_expr) => {
                let left_scalar = logical_expr.letf.interpret_checked(env.clone())?;
                let left_condition = (!!left_scalar.clone()).as_bool().unwrap();
                // println!("[left] {} {}",left_scalar,left_condition);
                if matches!(&logical_expr.operator.t_type, TokenType::OR) {
                    if left_condition {
                        left_scalar
                    } else {
                        logical_expr.right.interpret_checked(env.clone())?
                    }
                } else {
                    if left_condition {
                        logical_expr.right.interpret_checked(env.clone())?
                    } else {
                        left_scalar
                    }
                }
            }
            Expr::Call(call_expr) => {
                let scalar = call_expr.callee.interpret(env.clone())?;
                let expr_arr = &call_expr.arguments;
                let mut args = vec![];

                let maybe_fun = scalar.as_fun();
                let fun = match maybe_fun {
                    Some(f) => f,
                    None => {
                        return {
                            report_runtime(
                                call_expr.parent.line,
                                format!("Can only call functions and classes."),
                            );
                            InterpretRtErr!(;"Can only call functions and classes.")
                        }
                    }
                };

                if fun.arity() != expr_arr.len() {
                    let msg = format!(
                        "Expected {} arguments but got {}.",
                        fun.arity(),
                        expr_arr.len()
                    );
                    report_runtime(call_expr.parent.line, msg);
                    return InterpretRtErr!(;msg);
                }
                for expr in expr_arr {
                    args.push(expr.interpret(env.clone())?);
                }
                fun.call(args)?
            }
        };
        Ok(value)
    }
}
fn check_number_operands(left: &Scalar, right: &Scalar, operator: &Token) -> InterpretResult<()> {
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
