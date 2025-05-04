use std::path::PathBuf;

use bytes::Bytes;

use crate::{
    error::MyResult,
    evaluator::{Evaluator, InterpretError, Interprete},
    expr::Expr,
    parser::Parser,
    resolver::Resolver,
    scanner::Scanner,
    MyErr,
};

pub struct Lox {}
impl Lox {
    pub fn run_file(path: PathBuf, resolver: bool) -> MyResult<()> {
        let scanner = Self::tokenize(path);
        let mut parser = Parser::new(scanner.tokens());
        let stmts = parser.parse();
        let mut evaluator = Evaluator::new(resolver);
        if resolver {
            let mut resolver = Resolver::new(&mut evaluator);
            resolver.resolve_stmts(&stmts);
            // println!("[locals]: {:?}", evaluator.locals);
        }
        for stmt in stmts {
            let res = evaluator.eval(&stmt);
            if let Err(e) = res {
                match e {
                    InterpretError::Runtime(msg) => {
                        eprintln!("[runtime err] {}", msg);
                        return MyErr!(;msg)
                    },
                    _ => {
                        eprintln!("[stmt err] {:?}", e)
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
        let scanner = Self::tokenize(path);
        let mut parser = Parser::new(scanner.tokens());
        let expr = parser.parse_expression();
        let mut evaluator = Evaluator::new(false);

        if let Some(expr) = expr {
            let result = evaluator.eval(&expr);
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
