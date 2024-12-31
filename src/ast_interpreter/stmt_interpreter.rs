use crate::{
    data_types::scaler::{Scalar, UserFn},
    environment::{Environment, EnvironmentType},
    stmt::Stmt, InterpretRet,
};

use super::{
    interpret_err::{InterpretError, InterpretResult},
    AstInterpreter,
};

impl AstInterpreter for Stmt {
    type Output = InterpretResult<()>;

    fn interpret(&self, env: EnvironmentType) -> Self::Output {
        self.interpret_checked(env)
    }
}
impl Stmt {
    fn interpret_checked(&self, env: EnvironmentType) -> InterpretResult<()> {
        let value = match self {
            Stmt::Var(var_stmt) => {
                let var_stmt = var_stmt.clone();
                let name = var_stmt.name.lexeme;
                if let Some(x) = var_stmt.initializer {
                    let value = x.interpret(env.clone())?;
                    env.borrow_mut().define(name, Some(value));
                } else {
                    env.borrow_mut().define(name, None);
                }
            }
            Stmt::Expression(expression_stmt) => {
                expression_stmt.expression.interpret(env)?;
            }
            Stmt::Block(block_stmt) => {
                let env = Environment::new(Some(env), Some("block"));
                for i in &block_stmt.statements {
                    i.interpret_checked(env.clone())?
                }
            }
            Stmt::Print(print_stmt) => {
                let result = print_stmt.expression.interpret(env)?;

                println!("{}", result)
            }
            Stmt::If(if_stmt) => {
                let left_value: Scalar = if_stmt.condition.interpret(env.clone())?;
                let condition = (!!left_value.clone()).as_bool().unwrap();
                // println!("[if] {} {}", left_value, condition);

                if condition {
                    if_stmt.then_branch.interpret(env)?;
                } else {
                    if let Some(else_branch) = &if_stmt.else_branch {
                        else_branch.interpret(env)?;
                    }
                }
            }
            Stmt::While(while_stmt) => {
                while let Some(condition) = while_stmt.condition.interpret(env.clone())?.as_bool() {
                    if !condition {
                        break;
                    }
                    while_stmt.body.interpret(env.clone())?;
                }
            }
            Stmt::Function(function_stmt) => {
                let fun = *(function_stmt.clone());
                env.borrow_mut().define(
                    fun.name.lexeme.clone(),
                    Some(UserFn::new(env.clone(), fun).into()),
                );
            }
            Stmt::Return(return_stmt) => {
                if let Some(expr) = &return_stmt.value {
                    return InterpretRet!(expr.interpret(env)?);
                }
                return InterpretRet!(Scalar::Nil);
            }
        };
        Ok(value)
    }
}
