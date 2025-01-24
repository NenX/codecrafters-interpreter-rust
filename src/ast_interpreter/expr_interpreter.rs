use crate::{
    callable::Callable,
    data_types::scaler::Scalar,
    environment::{Environment, EnvironmentType},
    error::report_runtime,
    evaluator::Evaluator,
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
        Evaluator::interpret_expr(self, env)
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
