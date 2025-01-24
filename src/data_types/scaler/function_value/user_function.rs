
use crate::{
    callable::Callable,
    data_types::scaler::Scalar,
    environment::{Environment, EnvironmentType},
    evaluator::{Evaluator, InterpretError, InterpretResult},
    stmt::function::FunctionStmt,
};

#[derive(Debug, Clone)]

pub struct UserFn {
    closure: EnvironmentType,
    declaration: FunctionStmt,
}
impl UserFn {
    pub fn new(env: EnvironmentType, delc: FunctionStmt) -> Self {
        Self {
            closure: env,
            declaration: delc,
        }
    }
}
impl Callable for UserFn {
    fn to_string(&self) -> String {
        let name = &self.declaration.name.lexeme;
        format!("<fn {}>", name)
    }

    fn call(&self, evaluator: &mut Evaluator, args: Vec<Scalar>) -> InterpretResult<Scalar> {
        let env = Environment::new(
            Some(self.closure.clone()),
            Some(&self.declaration.name.lexeme),
        );

        for (idx, token) in self.declaration.params.iter().enumerate() {
            let mut env_mut = env.borrow_mut();

            env_mut.define(token.lexeme.clone(), args.get(idx).cloned());
        }
        // let res = self.declaration.body.interpret(env.clone());
        let res = evaluator.exit_block(&self.declaration.fn_body, env.clone());
        let ret = match res {
            Ok(_) => Scalar::Nil,
            Err(e) => match e {
                InterpretError::Return(scalar) => scalar,
                _ => return Err(e),
            },
        };
        Ok(ret)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}
