use crate::{
    ast_interpreter::interpret_err::InterpretResult, data_types::scaler::Scalar, error::MyResult,
};

pub trait Callable {
    fn to_string(&self) -> String {
        format!("<native fn>")
    }
    fn arity(&self) -> usize;

    fn call(&self, args: Vec<Scalar>) -> InterpretResult<Scalar>;
}

struct A;
impl A {
    fn say(&self) {
        println!("I'm A")
    }
}
impl MyTrait for A {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
trait MyTrait {
    fn as_any(&self) -> &dyn std::any::Any;
}
fn get_dyn() -> Box<dyn MyTrait> {
    Box::new(A)
}
#[test]
fn tt() {
    let dyn_obj: Box<dyn MyTrait> = get_dyn();
    let a: &A = dyn_obj.as_any().downcast_ref().unwrap();
    a.say();
}
