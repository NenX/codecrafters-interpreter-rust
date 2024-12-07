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

fn aa() -> MyResult<()> {
    Err(std::io::Error::new(std::io::ErrorKind::AddrInUse, "??xx??").into())
}

#[test]
fn tt() {
    let e = aa();
    match e {
        Ok(_) => todo!(),
        Err(e) => {
            let em = e.downcast::<MyErrImpl>();
        }
    }
}
