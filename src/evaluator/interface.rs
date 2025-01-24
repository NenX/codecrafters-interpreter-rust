use crate::environment::EnvironmentType;

pub trait Interprete<T> {
    type Output;
    fn eval(&mut self, value: &T,) -> Self::Output;
}

