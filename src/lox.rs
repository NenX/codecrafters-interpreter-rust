use std::{error::Error, path::PathBuf};

use bytes::Bytes;

use crate::{
    error::{MyErrImpl, MyResult},
    scanner::Scanner,
};

pub struct Lox {}
impl Lox {
    pub fn run_file(path: PathBuf) -> MyResult<()> {
        let r = std::fs::read(path).expect("read file");
        let b = bytes::Bytes::from(r);
        Self::run(b)
    }
    fn run(b: Bytes) -> MyResult<()> {
        let mut scanner = Scanner::new(b);
        scanner.scan_tokens().expect("scan_tokens");
        scanner.print_tokens();

        Ok(())
    }
}
