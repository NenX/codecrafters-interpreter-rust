use crate::{
    data_types::scaler::{ClassValue, Scalar, UserFn},
    environment::Environment,
    error::report_runtime,
    stmt::Stmt,
    InterpretRet, InterpretRtErr,
};

use super::{error::InterpretResult, Evaluator, InterpretError, Interprete};

impl Interprete<Stmt> for Evaluator {
    type Output = InterpretResult<()>;

    fn eval(&mut self, stmt: &Stmt) -> InterpretResult<()> {
        match stmt {
            Stmt::Var(var) => {
                let value = match &var.initializer {
                    Some(expr) => Some(self.eval(expr)?),
                    None => None,
                };
                self.env.borrow_mut().define(var.name.lexeme.clone(), value);
                Ok(())
            }
            Stmt::Expression(expr) => {
                self.eval(&expr.expression)?;
                Ok(())
            }

            Stmt::Print(print) => {
                let value = self.eval(&print.expression)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::If(if_stmt) => {
                let condition = self.eval(&if_stmt.condition)?;
                if (!!condition).as_bool().unwrap() {
                    self.eval(&if_stmt.then_branch)
                } else if let Some(else_branch) = &if_stmt.else_branch {
                    self.eval(else_branch)
                } else {
                    Ok(())
                }
            }
            Stmt::While(while_stmt) => {
                while let Some(condition) = self.eval(&while_stmt.condition)?.as_bool() {
                    if !condition {
                        break;
                    }
                    self.eval(&while_stmt.body)?;
                }
                Ok(())
            }
            Stmt::Function(func) => {
                let name = &func.name.lexeme;
                let fun = UserFn::new(self.env.clone(), func.clone());
                self.env.borrow_mut().define(name, Some(fun.into()));
                Ok(())
            }
            Stmt::Block(block) => self.eval_block(
                &block.statements,
                Environment::new(Some(self.env.clone()), None),
            ),
            Stmt::Return(ret) => {
                let value = match &ret.value {
                    Some(expr) => self.eval(expr)?,
                    None => Scalar::Nil,
                };
                InterpretRet!(value)
            }
            Stmt::Class(class) => {
                let enclosing_env = self.env.clone();
                let has_superclass = class.superclass.is_some();
                let name = &class.name.lexeme;

                let super_class = if let Some(super_class) = &class.superclass {
                    let super_value = self.eval(super_class).expect("superclass not found");

                    let Some(super_value) = super_value.as_class() else {
                        report_runtime(class.name.line, "Superclass must be a class.".to_string());
                        return InterpretRtErr!(;"superclass must be a class");
                    };

                    // let super_class = super_value.clone();

                    Some(super_value.clone())
                } else {
                    None
                };
                // if has_superclass {
                //     self.env = Environment::new(Some(enclosing_env.clone()), Some("super env"));
                //     self.env
                //         .borrow_mut()
                //         .define("super", Some(super_class.clone().unwrap().into()));
                // }
                let mut class_value = ClassValue::new(name, super_class.map(|s| s.into()));
                let fn_env = if has_superclass {
                    let super_env = Environment::new(Some(self.env.clone()), Some("super env"));
                    super_env
                        .borrow_mut()
                        .define("super", Some(class_value.clone().into()));
                    super_env
                } else {
                    self.env.clone()
                };
                for function in &class.methods {
                    let fun = UserFn::new(fn_env.clone(), function.clone());
                    class_value.add_method(&function.name.lexeme, fun);
                }
                self.env.borrow_mut().define(name, Some(class_value.into()));
                Ok(())
            }
        }
    }
}
