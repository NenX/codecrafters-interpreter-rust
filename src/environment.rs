use std::{
    borrow::Borrow, cell::RefCell, collections::HashMap, error::Error, fmt::Display, rc::Rc,
};

use crate::data_types::scaler::{NativeFn, Scalar};
pub type EnvironmentType = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub struct Environment {
    name: String,
    enclosing: Option<EnvironmentType>,
    values: HashMap<String, Scalar>,
}

impl Environment {
    pub fn new(enclosing: Option<EnvironmentType>, name: Option<&str>) -> EnvironmentType {
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
    pub fn assign(&mut self, name: impl AsRef<str>, value: Scalar) -> Result<(), EnvErr> {
        if self.values.contains_key(name.as_ref()) {
            self.values.insert(name.as_ref().to_string(), value);
            return Ok(());
        }
        match &self.enclosing {
            Some(parent) => parent.borrow_mut().assign(name, value),
            None => Err(EnvErr::AccessUndefined),
        }
    }
    pub fn get(&self, name: impl AsRef<str>) -> Result<Scalar, EnvErr> {
        if let Some(value) = self.values.get(name.as_ref()) {
            return Ok(value.clone());
        }
        match &self.enclosing {
            Some(parent) => {
                // parent.borrow().get(name);
                let a: &RefCell<Environment> = parent.borrow();
                let b = a.borrow();
                b.get(name)
            }
            None => Err(EnvErr::AccessUndefined),
        }
    }
    pub fn ancestor(&self, distance: usize) -> EnvironmentType {
        assert!(distance > 0);
        let mut env: EnvironmentType = self.enclosing.clone().unwrap();
        for _ in 1..distance {
            let _env: &RefCell<Environment> = env.borrow();
            let _env: EnvironmentType = _env.borrow().enclosing.clone().unwrap();
            env = _env;
        }

        env
    }

    pub fn get_at(&self, distance: usize, name: impl AsRef<str>) -> Result<Scalar, EnvErr> {
        if distance == 0 {
            return self.get(name);
        }
        self.ancestor(distance).borrow_mut().get(name)
    }
    pub fn assign_at(
        &mut self,
        distance: usize,
        name: impl AsRef<str>,
        value: Scalar,
    ) -> Result<(), EnvErr> {
        if distance == 0 {
            return self.assign(name, value);
        }
        self.ancestor(distance).borrow_mut().assign(name, value)
    }
}
impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = self.enclosing.borrow();
        if let Some(enclosing) = e {
            write!(
                f,
                "({},{:?},[{}])",
                self.name,
                self.values.keys(),
                enclosing.borrow_mut()
            )
        } else {
            write!(f, "({},{:?})", self.name, self.values.keys())
        }
    }
}
#[derive(Debug)]
pub enum EnvErr {
    AssignUndefined,
    AccessUndefined,
}
impl Display for EnvErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for EnvErr {}
