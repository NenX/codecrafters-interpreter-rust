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
    fn is_naive(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl From<&Box<dyn Callable>> for NativeFn {
    fn from(value: &Box<dyn Callable>) -> Self {
        let a: &Self = value.as_any().downcast_ref().unwrap();
        a.clone()
    }
}

#[test]
fn tt() {
    let a = SystemTime::now();
    let d = a.duration_since(SystemTime::UNIX_EPOCH).unwrap();

    println!("{:?}", d.as_millis())
}
