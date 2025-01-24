use crate::{
    data_types::scaler::{Scalar, UserFn},
    environment::{Environment, EnvironmentType},
    evaluator::Evaluator,
    stmt::Stmt,
    InterpretRet,
};

use super::{
    interpret_err::{InterpretError, InterpretResult},
    AstInterpreter,
};

impl AstInterpreter for Stmt {
    type Output = InterpretResult<()>;

    fn interpret(&self, env: EnvironmentType) -> Self::Output {
        Evaluator::interpret_stmt(self, env)
    }
}
