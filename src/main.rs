use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use bytes::Bytes;
use clap::Parser;
use codecrafters_interpreter::command::ArgsParser;
use codecrafters_interpreter::command::Cmd;
use codecrafters_interpreter::lox::Lox;
use codecrafters_interpreter::scanner::Scanner;

fn main() {
    let x = ArgsParser::parse();
    match x.cmds {
        Cmd::Tokenize { file } => {
            let res = Lox::run_file(file);
            if res.is_err() {
                process::exit(65)
            }
        }
    }
}
