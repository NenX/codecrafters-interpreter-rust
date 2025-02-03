use crate::{
   data_types::scaler::Scalar,
    evaluator::{Evaluator, InterpretResult},
};

pub trait Callable {
    fn to_string(&self) -> String {
        "<native fn>".to_string()
    }
    fn arity(&self) -> usize;

    fn call(&self, evaluator: &mut Evaluator, args: Vec<Scalar>) -> InterpretResult<Scalar>;
}


