mod native_function;
mod user_function;
pub use native_function::*;
pub use user_function::*;

use crate::{
    ast_interpreter::interpret_err::InterpretResult, callable::Callable,
    environment::EnvironmentType, error::MyResult,
};

use super::Scalar;

#[derive(Clone)]
pub enum FunctionValue {
    User(UserFn),
    Native(NativeFn),
}
impl From<UserFn> for FunctionValue {
    fn from(value: UserFn) -> Self {
        Self::User(value)
    }
}
impl From<NativeFn> for FunctionValue {
    fn from(value: NativeFn) -> Self {
        Self::Native(value)
    }
}
impl Callable for FunctionValue {
    fn call(&self, args: Vec<Scalar>) -> InterpretResult<Scalar> {
        match self {
            FunctionValue::User(user_fn) => user_fn.call(args),
            FunctionValue::Native(native_fn) => native_fn.call(args),
        }
    }
    fn to_string(&self) -> String {
        match self {
            FunctionValue::User(user_fn) => user_fn.to_string(),
            FunctionValue::Native(native_fn) => native_fn.to_string(),
        }
    }

    fn arity(&self) -> usize {
        match self {
            FunctionValue::User(user_fn) => user_fn.arity(),
            FunctionValue::Native(native_fn) => native_fn.arity(),
        }
    }
}
