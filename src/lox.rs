use std::{error::Error, path::PathBuf};

use bytes::Bytes;

use crate::{
    error::{MyErrImpl, MyResult},
    expr::Expr,
    parser::Parser,
    scanner::Scanner,
    token::Token,
};

pub struct Lox {}
impl Lox {
    pub fn run_file(path: PathBuf) -> MyResult<()> {
        let b = Self::read(path);
        Self::run(b)
    }
    pub fn parse(path: PathBuf) -> Option<Expr> {
        let scanner = Self::tokenize(path);
        let mut parser = Parser::new(scanner.tokens());
        parser.parse()
    }
    pub fn tokenize(path: PathBuf) -> Scanner {
        let b = Self::read(path);
        let mut scanner = Scanner::new(b);
        scanner.scan_tokens().expect("scan_tokens");
        // scanner.print_tokens();
        scanner
    }
    fn read(path: PathBuf) -> Bytes {
        let r = std::fs::read(path).expect("read file");
        let b = bytes::Bytes::from(r);
        eprintln!("files data: {:?}", b);
        b
    }
    fn run(b: Bytes) -> MyResult<()> {
        let mut scanner = Scanner::new(b);
        scanner.scan_tokens().expect("scan_tokens");
        scanner.print_tokens();

        Ok(())
    }
}
