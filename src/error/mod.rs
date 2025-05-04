use std::{error::Error, fmt::Display};

use crate::{token::Token, token_type::TokenType};

pub type MyResult<T> = Result<T, anyhow::Error>;
pub type MyError = anyhow::Error;

#[macro_export]

macro_rules! MyErr {
    ($x:expr) => {
        anyhow::Error::msg($x)
    };
    (;$x:expr) => {
        Err(anyhow::Error::msg($x))
    };
    (,$x:expr) => {
        Err($x.into())
    };
}

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

pub static mut HAD_ERROR: bool = false;
pub static mut HAD_RUNTIME_ERROR: bool = false;

pub fn my_error_token(token: Token, message: String) {
    if token.t_type == TokenType::EOF {
        report(token.line, " at end".to_string(), message);
    } else {
        report(token.line, format!(" at '{}'", token.lexeme), message);
    }
}
pub fn my_error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

pub fn unexpected_terminal_err(line: usize) {
    my_error(line, "Unterminated string.".to_string());
}

fn report(line: usize, r#where: String, message: String) {
    let msg = format!("[line {}] Error{}: {}", line, r#where, message);
    eprintln!("{}", msg);
    unsafe {
        HAD_ERROR = true;
    };
}
pub fn report_runtime(line: usize, message: String) {
    let msg = format!("{}[line {}] ", message, line,);

    eprintln!("{}", msg);
    unsafe {
        HAD_RUNTIME_ERROR = true;
    };
}

pub fn had_error() -> bool {
    unsafe { HAD_ERROR }
}
