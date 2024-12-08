use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use bytes::Bytes;
use clap::Parser;
use codecrafters_interpreter::command::ArgsParser;
use codecrafters_interpreter::command::Cmd;
use codecrafters_interpreter::error::HAD_ERROR;
use codecrafters_interpreter::lox::Lox;
use codecrafters_interpreter::scanner::Scanner;

fn main() {
    let x = ArgsParser::parse();
    match x.cmds {
        Cmd::Tokenize { file } => {
            Lox::run_file(file).expect("run file");
            if unsafe { HAD_ERROR } {
                process::exit(65)
            }
        }
    }
}
