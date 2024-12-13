use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use bytes::Bytes;
use clap::Parser;
use codecrafters_interpreter::command::ArgsParser;
use codecrafters_interpreter::command::Cmd;
use codecrafters_interpreter::error::HAD_ERROR;
use codecrafters_interpreter::expr::ast_printer::AstPrinter;
use codecrafters_interpreter::lox::Lox;

fn main() {
    let x = ArgsParser::parse();
    match x.cmds {
        Cmd::Tokenize { file } => {
            Lox::tokenize(file).print_tokens();
        }
        Cmd::Parse { file } => {
            let opt = Lox::parse(file);

            opt.and_then(|expr| {
                let s = expr.print();
                println!("{}", s);
                Some(s)
            });
        }
    }
    if unsafe { HAD_ERROR } {
        process::exit(65)
    }
}
