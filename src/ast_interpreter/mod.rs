use crate::environment::Environment;

mod expr_interpreter;
mod stmt_interpreter;

pub trait AstInterpreter {
    type Output;
    fn interpret(&self, env:&mut Environment) -> Self::Output;
}
