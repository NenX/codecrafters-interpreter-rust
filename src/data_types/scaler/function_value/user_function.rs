use std::{cell::RefCell, rc::Rc};

use crate::{
    callable::Callable,
    data_types::scaler::{InstanceValue, Scalar},
    environment::{Environment, EnvironmentType},
    evaluator::{Evaluator, InterpretError, InterpretResult},
    stmt::function::FunctionStmt,
};

#[derive(Debug, Clone)]

pub struct UserFn {
    pub instance: Option<Rc<RefCell<Scalar>>>, // 是否 bind 过
    closure: EnvironmentType,
    declaration: Rc<FunctionStmt>,
}
impl UserFn {
    pub fn new(env: EnvironmentType, delc: Rc<FunctionStmt>) -> Self {
        Self {
            closure: env,
            declaration: delc,
            instance: None,
        }
    }
    pub fn bind(&self, instance: Scalar) -> Self {
        let env = Environment::new(Some(self.closure.clone()), Some("bind env"));
        env.borrow_mut().define("this", Some(instance.clone()));
        Self {
            closure: env,
            declaration: self.declaration.clone(),
            instance: Some(Rc::new(RefCell::new(instance))),
        }
    }
    pub fn is_init(&self) -> bool {
        self.declaration.name.lexeme == "init"
    }
}
impl Callable for UserFn {
    fn to_string(&self) -> String {
        let name = &self.declaration.name.lexeme;
        format!("<fn {}>", name)
    }

    fn call(&self, evaluator: &mut Evaluator, args: Vec<Scalar>) -> InterpretResult<Scalar> {
        let env = Environment::new(Some(self.closure.clone()), Some(&self.to_string()));

        for (idx, token) in self.declaration.params.iter().enumerate() {
            let mut env_mut = env.borrow_mut();

            env_mut.define(token.lexeme.clone(), args.get(idx).cloned());
        }
        // let res = self.declaration.body.interpret(env.clone());
        // println!("call user function: {} env: {}", self.to_string(), env.borrow());
        let res = evaluator.eval_block(&self.declaration.fn_body, env.clone());
        let Err(e) = res else {
            if self.is_init() && self.instance.is_some() {
                return Ok(self.instance.as_ref().unwrap().borrow().clone());
            }
            return Ok(Scalar::Nil);
        };
        let ret = match e {
            InterpretError::Return(scalar) => scalar,
            _ => return Err(e),
        };
        Ok(ret)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}
