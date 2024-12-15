use crate::{
    data_types::scaler::Scalar,
    error::{report_runtime, MyResult},
    expr::binary::Binary,
    token::Token,
    token_type::TokenType,
    MyErr,
};

use super::Expr;

pub trait AstInterpreter {
    fn interpret_checked(&self) -> MyResult<Scalar>;
    fn interpret(&self) -> Scalar {
        let scaler = match self.interpret_checked() {
            Ok(sc) => {
                println!("{}", sc);
                sc
            }
            Err(e) => Scalar::Nil,
        };
        scaler
    }
}
impl AstInterpreter for Expr {
    fn interpret_checked(&self) -> MyResult<Scalar> {
        let value = match self {
            Expr::Binary(binary) => {
                let Binary {
                    letf,
                    right,
                    operator,
                } = binary.as_ref();
                let left = letf.interpret_checked()?;
                let right = right.interpret_checked()?;
                match operator.token_type {
                    TokenType::MINUS => {
                        check_number_operands(&left, &right, &operator)?;
                        left + right
                    }
                    TokenType::PLUS => {
                        check_number_operands(&left, &right, &operator)?;
                        left + right
                    }
                    TokenType::SLASH => {
                        check_number_operands(&left, &right, &operator)?;
                        left + right
                    }
                    TokenType::STAR => {
                        check_number_operands(&left, &right, &operator)?;
                        left + right
                    }

                    TokenType::BangEqual => {
                        check_number_operands(&left, &right, operator)?;
                        Scalar::Bool(left != right)
                    }
                    TokenType::EqualEqual => {
                        check_number_operands(&left, &right, operator)?;
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
            Expr::Grouping(grouping) => grouping.expression.interpret_checked()?,
            Expr::Literal(literal) => literal.value.clone(),
            Expr::Unary(unary) => match unary.operator.token_type {
                TokenType::LeftParen => !unary.right.interpret_checked()?,
                TokenType::BANG => {
                    let right = unary.right.interpret_checked()?;
                    check_number_operand(&right, &unary.operator)?;
                    -unary.right.interpret_checked()?
                }
                _ => Scalar::Nil,
            },
        };
        Ok(value)
    }
}
fn check_number_operands(left: &Scalar, right: &Scalar, operator: &Token) -> MyResult<()> {
    if Scalar::check_number_operands(left, right) {
        Ok(())
    } else {
        report_runtime(operator.line, format!("Operands must be numbers."));
        MyErr!(;"need")
    }
}
fn check_number_operand(right: &Scalar, operator: &Token) -> MyResult<()> {
    if matches!(right, Scalar::Number(_)) {
        Ok(())
    } else {
        report_runtime(operator.line, format!("Operand must be a number."));
        MyErr!(;"need")
    }
}
