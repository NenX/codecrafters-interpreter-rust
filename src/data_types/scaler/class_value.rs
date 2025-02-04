use std::{collections::HashMap, rc::Rc};

use crate::{
    callable::Callable,
    evaluator::{Evaluator, InterpretResult},
};

use super::{InstanceValue, Scalar, UserFn};

#[derive(Clone)]
pub struct ClassValue {
    pub name: String,
    pub methods: HashMap<String, UserFn>,
    pub super_class: Option<Rc<ClassValue>>,
}
impl PartialEq for ClassValue {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl ClassValue {
    pub fn new(name: &str, super_class: Option<Rc<ClassValue>>) -> Self {
        Self {
            name: name.to_string(),
            methods: HashMap::new(),
            super_class,
        }
    }
    pub fn find_method(&self, name: &str) -> Option<UserFn> {
        let method = self.methods.get(name);
        if let Some(method) = method {
            return Some(method.clone());
        }
        if let Some(super_class) = &self.super_class {
            super_class.find_method(name)
        } else {
            None
        }
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
