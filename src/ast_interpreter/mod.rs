use crate::environment::EnvironmentType;

pub mod expr_interpreter;
pub mod interpret_err;
pub mod stmt_interpreter;

pub trait AstInterpreter {
    type Output;
    fn interpret(&self, env: EnvironmentType) -> Self::Output;
}
