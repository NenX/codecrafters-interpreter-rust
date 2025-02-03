use std::rc::Rc;

use crate::{
    data_types::scaler::{ClassValue, FunctionValue, Scalar, UserFn},
    environment::Environment,
    stmt::Stmt,
    InterpretRet,
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
                let name = &class.name.lexeme;
                let mut class_value = ClassValue::new(name);

                for function in &class.methods {
                    let fun = UserFn::new(self.env.clone(), function.clone());
                    class_value
                        .methods
                        .insert(function.name.lexeme.clone(), fun);
                }

                self.env.borrow_mut().define(name, Some(class_value.into()));
                Ok(())
            }
        }
    }
}
