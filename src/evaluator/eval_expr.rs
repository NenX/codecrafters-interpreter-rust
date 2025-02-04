
use crate::{
    callable::Callable,
    data_types::scaler::Scalar,
    error::report_runtime,
    expr::{binary::BinaryExpr, Expr},
    token_type::TokenType,
    InterpretRtErr,
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
                let name = &variable.name.lexeme;
                let value = self.lookup_variable(expr, name);
                match value {
                    Ok(value) => Ok(value.clone()),
                    Err(_) => {
                        report_runtime(
                            variable.name.line,
                            format!("Access undefined variable '{}'.", name),
                        );
                        InterpretRtErr!(;"bad variable access")
                    }
                }
            }
            Expr::Assign(assign) => {
                let value = self.eval(&assign.value)?;
                let name = &assign.name.lexeme;

                match self.assign_variable(expr, name, value.clone()) {
                    Ok(_) => Ok(value),
                    Err(_) => {
                        report_runtime(
                            assign.name.line,
                            format!("Assign to undefined variable '{}'.", name),
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

                let function = callee.as_callable().ok_or_else(|| {
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
            Expr::Get(get) => {
                let object = self.eval(&get.object)?;
                let instance = object.as_instance();
                if let Some(instance) = instance {
                    instance.borrow().get(&get.name)
                } else {
                    report_runtime(get.name.line, "Only instances have properties.".to_string());
                    InterpretRtErr!(;"bad get")
                }
            }
            Expr::Set(set) => {
                let object = self.eval(&set.object)?;
                let value = self.eval(&set.value)?;
                let instance = object.as_instance();
                if let Some(instance) = &instance {
                    // instance.set(name, value.clone());
                    instance.borrow_mut().set(&set.name, value.clone());
                    Ok(value)
                } else {
                    report_runtime(set.name.line, "Only instances have properties.".to_string());
                    InterpretRtErr!(;"bad set")
                }
            }
            Expr::This(this) => {
                let name = &this.keyword.lexeme;
                let value = self.lookup_variable(expr, name);
                match value {
                    Ok(value) => Ok(value.clone()),
                    Err(_) => {
                        report_runtime(this.keyword.line, "Undefined variable 'this'.".to_string());
                        InterpretRtErr!(;"bad this")
                    }
                }
            }
            Expr::Super(super_expr) => {
                let method_name = &super_expr.method.lexeme;
                let distance = self
                    .get_depth(expr)
                    .expect("Superclass distance not found.");
                let sup_class = self
                    .env
                    .borrow()
                    .get_at(distance, "super")
                    .expect("super not found.");
                let sup_class = sup_class.as_class().expect("super is not a class.");
                let sup_method = sup_class.find_method(method_name);
                let this_instance = self
                    .env
                    .borrow()
                    .get_at(distance - 1, "this")
                    .expect("this not found.")
                    .as_instance()
                    .expect("this is not an instance.");
                match sup_method {
                    Some(sup_method) => Ok(sup_method.bind(&this_instance.borrow()).into()),
                    None => {
                        report_runtime(
                            super_expr.keyword.line,
                            "Undefined variable 'super'.".to_string(),
                        );
                        InterpretRtErr!(;"bad super")
                    }
                }
            }
        }
    }
}
