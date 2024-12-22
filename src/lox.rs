use std::path::PathBuf;

use bytes::Bytes;

use crate::{
    ast_interpreter::AstInterpreter, environment::Environment, expr::Expr, parser::Parser,
    scanner::Scanner, stmt::Stmt,
};
pub struct Lox {}
impl Lox {
    pub fn run_file(path: PathBuf) {
        let b = Self::read(path);
        Self::run(b)
    }
    pub fn parse(path: PathBuf) -> Vec<Stmt> {
        let scanner = Self::tokenize(path);
        let mut parser = Parser::new(scanner.tokens());
        parser.parse()
    }
    pub fn evaluate(path: PathBuf) {
        let stmts = Self::parse(path);
        let mut env = Environment::new(None);

        stmts.iter().for_each(|stmt| stmt.interpret(&mut env));
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
    fn run(b: Bytes) {
        let mut scanner = Scanner::new(b);
        scanner.scan_tokens().expect("scan_tokens");
        // scanner.print_tokens();
    }
}
