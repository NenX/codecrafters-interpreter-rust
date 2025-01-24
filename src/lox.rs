use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::LazyLock};

use bytes::Bytes;

use crate::{
    environment::Environment,
    error::MyResult,
    evaluator::{AstInterpreter, Evaluator, InterpretError},
    expr::Expr,
    parser::Parser,
    scanner::Scanner,
    MyErr,
};

pub struct Lox {}
impl Lox {
    pub fn run_file(path: PathBuf) -> MyResult<()> {
        let scanner = Self::tokenize(path);
        let mut parser = Parser::new(scanner.tokens());
        let stmts = parser.parse();
        let mut evaluator = Evaluator::new();
        let env = Environment::global_env();
        for stmt in stmts {
            let res = evaluator.eval(&stmt, env.clone());
            if let Err(e) = res {
                match e {
                    InterpretError::Runtime(msg) => return MyErr!(;msg),
                    _ => {
                        println!("stmt err {:?}", e)
                    }
                }
            }
        }
        Ok(())
    }
    pub fn parse(path: PathBuf) -> Option<Expr> {
        let scanner = Self::tokenize(path);
        let mut parser = Parser::new(scanner.tokens());
        parser.parse_expression()
    }
    pub fn evaluate(path: PathBuf) {
        let env = Environment::new(None, None);

        let scanner = Self::tokenize(path);
        let mut parser = Parser::new(scanner.tokens());
        let expr = parser.parse_expression();
        let mut evaluator = Evaluator::new();

        if let Some(expr) = expr {
            let result = evaluator.eval(&expr, env);
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
