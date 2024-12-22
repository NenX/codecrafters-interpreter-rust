use crate::{
    data_types::scaler::Scalar,
    environment::{Environment, EnvironmentType},
    error::{report_runtime, MyResult},
    expr::{binary::BinaryExpr, Expr},
    token::Token,
    token_type::TokenType,
    MyErr,
};

use super::AstInterpreter;

impl AstInterpreter for Expr {
    type Output = MyResult<Scalar>;

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
    fn interpret_checked(&self, env: EnvironmentType) -> MyResult<Scalar> {
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

                            return MyErr!(;"bad eval");
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
                            format!("Undefined variable '{}'.", variable.name.lexeme),
                        );
                        return MyErr!(;"bad variable access");
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
                            format!("Undefined variable '{}'.", assign.name.lexeme),
                        );
                        return MyErr!(;"bad variable assign");
                    }
                }
            }
        };
        Ok(value)
    }
}
fn check_number_operands(left: &Scalar, right: &Scalar, operator: &Token) -> MyResult<()> {
    if Scalar::check_number_operands(left, right) {
        Ok(())
    } else {
        report_runtime(operator.line, format!("Operands must be numbers."));
        MyErr!(;"bad eval")
    }
}
fn check_number_operand(right: &Scalar, operator: &Token) -> MyResult<()> {
    if matches!(right, Scalar::Number(_)) {
        Ok(())
    } else {
        report_runtime(operator.line, format!("Operand must be a number."));
        MyErr!(;"bad eval")
    }
}
