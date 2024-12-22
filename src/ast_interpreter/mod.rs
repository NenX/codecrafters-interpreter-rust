use crate::environment::{Environment, EnvironmentType};

mod expr_interpreter;
mod stmt_interpreter;

pub trait AstInterpreter {
    type Output;
    fn interpret(&self, env: EnvironmentType) -> Self::Output;
}
