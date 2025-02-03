use std::collections::HashMap;

use crate::{
    callable::Callable,
    data_types::scaler::FunctionValue,
    evaluator::{Evaluator, InterpretResult},
};

use super::{InstanceValue, Scalar, UserFn};

#[derive(Clone)]
pub struct ClassValue {
    pub name: String,
    pub methods: HashMap<String, UserFn>,
}
impl PartialEq for ClassValue {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl ClassValue {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            methods: HashMap::new(),
        }
    }
    pub fn find_method(&self, name: &str) -> Option<UserFn> {
        self.methods.get(name).cloned()
    }
}
impl Callable for ClassValue {
    fn to_string(&self) -> String {
        format!("<class {}>", self.name)
    }

    fn arity(&self) -> usize {
        if let Some(method) = self.methods.get("init") {
            method.arity()
        } else {
            0
        }
    }

    fn call(&self, evaluator: &mut Evaluator, args: Vec<Scalar>) -> InterpretResult<Scalar> {
        let instance = InstanceValue::new(self.clone());
        if let Some(method) = self.methods.get("init") {
            let a = method.call(evaluator, args)?;
            return Ok(a);
        }
        Ok(instance.into())
    }
}
