use std::{error::Error, fmt::Display};

pub type MyResult<T> = Result<T, anyhow::Error>;
pub type MyErr = anyhow::Error;

#[derive(Debug)]
pub struct MyErrImpl {}
impl Error for MyErrImpl {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let e = anyhow::Error::msg("message");
        e.downcast::<std::io::Error>();
        None
    }
}
impl Display for MyErrImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<MyErrImpl>")
    }
}

static mut HAD_ERROR: bool = false;
pub fn my_error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: usize, r#where: String, message: String) {
    let msg = format!("[line {}] Error{}: {}", line, r#where, message);
    println!("{}", msg);
    unsafe {
        HAD_ERROR = true;
    };
}

#[test]
fn tt() {}
