use crate::{
    ast_printer::AstPrinter,
    data_types::scaler::Scalar,
    environment::Environment,
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

    fn interpret(&self, env: &mut Environment) -> Self::Output {
        self.interpret_checked(env)
    }
}
impl Stmt {
    fn interpret_checked(&self, env: &mut Environment) -> MyResult<()> {
        let value = match self {
            Stmt::Var(var_stmt) => {
                let var_stmt = var_stmt.clone();
                let name = var_stmt.name.lexeme;
                if let Some(x) = var_stmt.initializer {
                    let value = x.interpret(env)?;
                    env.define(name, Some(value));
                } else {
                    env.define(name, None);
                }
            }
            Stmt::Expression(expression_stmt) => {
                expression_stmt.expression.interpret(env)?;
            }
            Stmt::Block(block_stmt) => {
                let mut env = Environment::new(Some(env));
                for i in &block_stmt.statements {
                    i.interpret_checked(&mut env)?
                }
            }
            Stmt::Print(print_stmt) => {
                let result = print_stmt.expression.interpret(env)?;

                println!("{}", result)
            }
        };
        Ok(value)
    }
}
