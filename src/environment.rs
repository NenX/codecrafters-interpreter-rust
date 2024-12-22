use std::{
    cell::RefCell, collections::HashMap, error::Error, fmt::Display, rc::Rc, sync::LazyLock,
};

use crate::data_types::scaler::Scalar;

pub struct Environment {
    enclosing: Option<EnvironmentType>,
    values: HashMap<String, Scalar>,
}
pub type EnvironmentType = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new(enclosing: Option<EnvironmentType>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            enclosing,
            values: HashMap::new(),
        }))
    }
    pub fn define(&mut self, name: String, value: Option<Scalar>) {
        self.values.insert(name, value.unwrap_or(Scalar::Nil));
    }
    pub fn assign(&mut self, name: String, value: Scalar) -> Result<(), EnvironmentErr> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return Ok(());
        }
        match &self.enclosing {
            Some(parent) => parent.borrow_mut().assign(name, value),
            None => Err(EnvironmentErr::AccessUndefined),
        }
    }
    pub fn get(&self, name: &String) -> Result<Scalar, EnvironmentErr> {
        if self.values.contains_key(name) {
            return Ok(self.values.get(name).unwrap().clone());
        }
        match &self.enclosing {
            Some(parent) => parent.borrow().get(name),
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
