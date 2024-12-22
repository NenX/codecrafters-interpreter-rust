use std::path::PathBuf;

use bytes::Bytes;

use crate::{
    ast_interpreter::AstInterpreter, data_types::scaler::Scalar, environment::Environment,
    expr::Expr, parser::Parser, scanner::Scanner, stmt::Stmt,
};
pub struct Lox {}
impl Lox {
    pub fn run_file(path: PathBuf) {
        let scanner = Self::tokenize(path);
        let mut parser = Parser::new(scanner.tokens());
        let stmts = parser.parse();
        let mut env = Environment::new(None);

        stmts.iter().for_each(|stmt| stmt.interpret(&mut env));
    }
    pub fn parse(path: PathBuf) -> Option<Expr> {
        let scanner = Self::tokenize(path);
        let mut parser = Parser::new(scanner.tokens());
        parser.parse_expression()
    }
    pub fn evaluate(path: PathBuf) {
        let mut env = Environment::new(None);

        let scanner = Self::tokenize(path);
        let mut parser = Parser::new(scanner.tokens());
        let expr = parser.parse_expression();
        if let Some(expr) = expr {
            let result = expr.interpret(&mut env);
            if let Ok(sc) = result {
                println!("{}", sc)
            }
        }
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
