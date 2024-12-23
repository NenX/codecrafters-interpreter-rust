use crate::{
    ast_printer::AstPrinter,
    data_types::scaler::Scalar,
    environment::{Environment, EnvironmentType},
    error::{report_runtime, MyResult},
    expr::{binary::BinaryExpr, Expr},
    stmt::Stmt,
    token::Token,
    token_type::TokenType,
    MyErr,
};

use super::AstInterpreter;

impl AstInterpreter for Stmt {
    type Output = MyResult<()>;

    fn interpret(&self, env: EnvironmentType) -> Self::Output {
        self.interpret_checked(env)
    }
}
impl Stmt {
    fn interpret_checked(&self, env: EnvironmentType) -> MyResult<()> {
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
                let mut env = Environment::new(Some(env));
                for i in &block_stmt.statements {
                    i.interpret_checked(env.clone())?
                }
            }
            Stmt::Print(print_stmt) => {
                let result = print_stmt.expression.interpret(env)?;

                println!("{}", result)
            }
            Stmt::If(if_stmt) => {
                let condition = if_stmt.condition.interpret(env.clone())?.as_bool().unwrap();

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
        };
        Ok(value)
    }
}
