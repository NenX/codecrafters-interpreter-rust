#[derive(Debug)]
struct SelfRef {}
impl SelfRef {
    fn new() -> Self {
        let s = Self {};
        println!("ptr inner {:?}", &s as *const SelfRef);
        s
    }
}
fn main() {
    let s = SelfRef::new();
    println!("ptr outer {:?}", &s as *const SelfRef);
}