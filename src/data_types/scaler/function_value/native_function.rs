use std::time::SystemTime;

use crate::{
    ast_interpreter::interpret_err::{InterpretError, InterpretResult},
    callable::Callable,
    data_types::scaler::Scalar,
    error::MyResult,
    InterpretRet,
};

#[derive(Debug, Clone)]

pub enum NativeFn {
    Clock,
    Log,
}

impl Callable for NativeFn {
    fn call(&self, args: Vec<Scalar>) -> InterpretResult<Scalar> {
        let duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let value = match self {
            NativeFn::Clock => Scalar::Number(duration.as_secs_f64() as f64),
            NativeFn::Log => Scalar::Nil,
        };
        Ok(value)
    }
    
    fn arity(&self) -> usize {
        0
    }

}


