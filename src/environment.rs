use std::{collections::HashMap, error::Error, fmt::Display, sync::LazyLock};

use crate::data_types::scaler::Scalar;
pub static mut GLOBAL_ENV: LazyLock<Environment> = LazyLock::new(|| Environment::new(None));

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Scalar>,
}
impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Self {
            enclosing: enclosing.map(|e| e.into()),
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: Option<Scalar>) {
        self.values.insert(name, value.unwrap_or(Scalar::Nil));
    }
    pub fn assign(&mut self, name: String, value: Scalar) -> Result<(), EnvironmentErr> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            Err(EnvironmentErr::AssignUndefined)
        }
    }
    pub fn get(&self, name: &String) -> Result<&Scalar, EnvironmentErr> {
        if self.values.contains_key(name) {
            return Ok(self.values.get(name).unwrap());
        }
        match &self.enclosing {
            Some(parent) => parent.get(name),
            None => Err(EnvironmentErr::AccessUndefined),
        }
    }
}

#[derive(Debug)]
pub enum EnvironmentErr {
    AssignUndefined,
    AccessUndefined,
}
impl Display for EnvironmentErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for EnvironmentErr {}
