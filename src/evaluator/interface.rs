use crate::environment::EnvironmentType;

pub trait AstInterpreter<T> {
    type Output;
    fn eval(&mut self, value: &T, env: EnvironmentType) -> Self::Output;
}

