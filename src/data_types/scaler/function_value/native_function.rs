use std::time::SystemTime;

use crate::{
    callable::Callable,
    data_types::scaler::Scalar,
    evaluator::{Evaluator, InterpretResult},
};

#[derive(Debug, Clone)]

pub enum NativeFn {
    Clock,
    Log,
}

impl Callable for NativeFn {
    fn call(&self, evaluator: &mut Evaluator, args: Vec<Scalar>) -> InterpretResult<Scalar> {
        let duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let value = match self {
            NativeFn::Clock => Scalar::Number(duration.as_secs_f64()),
            NativeFn::Log => Scalar::Nil,
        };
        Ok(value)
    }

    fn arity(&self) -> usize {
        0
    }
}
