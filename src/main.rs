use std::process;

use clap::Parser;
use codecrafters_interpreter::ast_printer::AstPrinter;
use codecrafters_interpreter::command::ArgsParser;
use codecrafters_interpreter::command::Cmd;
use codecrafters_interpreter::error::HAD_ERROR;
use codecrafters_interpreter::error::HAD_RUNTIME_ERROR;
use codecrafters_interpreter::lox::Lox;

fn main() {
    let x = ArgsParser::parse();
    match x.cmds {
        Cmd::Tokenize { file } => {
            Lox::tokenize(file).print_tokens();
        }
        Cmd::Parse { file } => {
            let opt = Lox::parse(file);

            opt.map(|expr| {
                let s = expr.print(true);
                println!("{}", s);
                s
            });
        }
        Cmd::Evaluate { file } => {
            Lox::evaluate(file);
        }
        Cmd::Run { file } => {
            let res = Lox::run_file(file);
        }
    }
    unsafe {
        eprintln!(
            "HAD_RUNTIME_ERROR {} HAD_ERROR {}",
            HAD_RUNTIME_ERROR, HAD_ERROR
        );
    }
    if unsafe { HAD_ERROR } {
        process::exit(65)
    }
    if unsafe { HAD_RUNTIME_ERROR } {
        process::exit(70)
    }
}

struct A {
    pub age: i32,
}
#[test]
fn aa() {
    let a = Rc::new(RefCell::new(A { age: 12 }));
    let b = a.clone();
    let c = a.clone();
    c.borrow_mut().age = 33;
    println!("a {}", a.borrow().age)
}
