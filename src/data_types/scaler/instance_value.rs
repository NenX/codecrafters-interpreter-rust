use std::collections::HashMap;

use crate::{
    error::report_runtime,
    evaluator::{InterpretError, InterpretResult},
    token::Token,
};

use super::{ClassValue, Scalar};

#[derive(Clone)]
pub struct InstanceValue {
    pub class: ClassValue,
    fields: HashMap<String, Scalar>,
}
impl InstanceValue {
    pub fn new(class: ClassValue) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }
    pub fn get(&self, token: &Token) -> InterpretResult<Scalar> {
        let name = &token.lexeme;
        let field = self.fields.get(name);
        if let Some(field) = field {
            return Ok(field.clone());
        }

        let method = self.class.methods.get(name);

        if let Some(method) = method {
            return Ok(method.bind(self).into());
        }
        report_runtime(token.line, format!("Field {} not found", name));
        Err(InterpretError::Runtime(format!("Field {} not found", name)))
    }
    pub fn set(&mut self, token: &Token, value: Scalar) {
        let name = &token.lexeme;
        self.fields.insert(name.to_string(), value);
    }
}
impl PartialEq for InstanceValue {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class
    }
}
