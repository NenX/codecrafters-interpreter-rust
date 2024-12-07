use std::env;
use std::fs;
use std::io::{self, Write};

use bytes::Bytes;
use clap::Parser;
use codecrafters_interpreter::command::ArgsParser;
use codecrafters_interpreter::command::Cmd;
use codecrafters_interpreter::scanner::Scanner;

fn main() {
    let x = ArgsParser::parse();
    match x.cmds {
        Cmd::Tokenize { file } => {
            let file = std::fs::read(file).expect("read file");
            let b = Bytes::from(file);
            let mut scanner = Scanner::new(b);
            scanner.scan_tokens().expect("scan_tokens");
            scanner.print_tokens();

            println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
        }
    }
}
