pub mod command;
pub mod constants;
pub mod error;
pub mod lox;
pub mod scanner;
pub mod parser;
pub mod expr;
pub mod token;
pub mod token_display;
pub mod token_type;
pub mod data_types;

trait Visit<T = ()> {
    fn visit_aa(&self, a: &Aa) -> T;
    fn visit_bb(&self, b: &Bb) -> T;
}
struct PrintVisit;
impl Visit<String> for PrintVisit {
    fn visit_aa(&self, a: &Aa) -> String {
        format!("(name: {})", a.name)
    }

    fn visit_bb(&self, b: &Bb) -> String {
        format!("(age: {})", b.age)
    }
}
trait Base {
    fn access<T>(&self, v: &impl Visit<T>) -> T;
}
struct Aa {
    name: String,
}
struct Bb {
    age: u32,
}
impl Base for Aa {
    fn access<T>(&self, v: &impl Visit<T>) -> T {
        v.visit_aa(self)
    }
}
impl Base for Bb {
    fn access<T>(&self, v: &impl Visit<T>) -> T {
        v.visit_bb(self)
    }
}
#[test]
fn test_visit() {
    let a = Aa {
        name: "I'm a".to_owned(),
    };
    let b = Bb { age: 22 };
    let v = PrintVisit;
    let a_str = a.access(&v);
    let b_str = b.access(&v);
    println!("a => {a_str}");
    println!("b => {b_str}");
}
