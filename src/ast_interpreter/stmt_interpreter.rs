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
    type Output = ();

    fn interpret(&self, env: &mut Environment) -> Self::Output {
        let scaler = match self.interpret_checked(env) {
            Ok(sc) => sc,
            Err(e) => (),
        };
        ()
    }
}
impl Stmt {
    fn interpret_checked(&self, env: &mut Environment) -> MyResult<()> {
        let value = match self {
            Stmt::Var(var_stmt) => {
                let var_stmt = var_stmt.clone();
                let name = var_stmt.name.lexeme;
                let value = var_stmt.initializer.map(|x| x.interpret(env));

                env.define(name, value);
            }
            Stmt::Expression(expression_stmt) => {
                expression_stmt.expression.interpret(env);
            }
            Stmt::Block(block_stmt) => {
                for i in &block_stmt.statements {
                    i.interpret_checked(env)?
                }
            }
            Stmt::Print(print_stmt) => {
                let result = print_stmt.expression.interpret(env);

                println!("{}", result)
            }
        };
        Ok(value)
    }
}
