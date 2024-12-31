use std::{
    borrow::Borrow, cell::RefCell, collections::HashMap, error::Error, fmt::Display, rc::Rc,
    sync::LazyLock,
};

use crate::{
    callable::Callable,
    data_types::scaler::{FunctionValue, NativeFn, Scalar},
};
pub type EnvironmentType = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub struct Environment {
    name: String,
    enclosing: Option<EnvironmentType>,
    values: HashMap<String, Scalar>,
}

impl Environment {
    pub fn new(enclosing: Option<EnvironmentType>, name: Option<&str>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: name.as_ref().unwrap_or(&"default").to_string(),
            enclosing,
            values: HashMap::new(),
        }))
    }
    pub fn global_env() -> EnvironmentType {
        let mut global = Self {
            name: "global".to_string(),
            enclosing: None,
            values: HashMap::new(),
        };

        global.define("clock", Some(NativeFn::Clock.into()));
        global.define("log", Some(NativeFn::Log.into()));
        Rc::new(RefCell::new(global))
    }
    pub fn define<T: AsRef<str>>(&mut self, name: T, value: Option<Scalar>) {
        self.values
            .insert(name.as_ref().to_string(), value.unwrap_or(Scalar::Nil));
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
            Some(parent) => {
                // parent.borrow().get(name);
                let a: &RefCell<Environment> = parent.borrow();
                let b = a.borrow();
                b.get(name)
            }
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
