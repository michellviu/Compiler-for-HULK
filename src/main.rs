mod ast;
mod error;
mod lexer;
mod parser;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.hulk>", args[0]);
        return;
    }

    let source = fs::read_to_string(&args[1]).expect("Failed to read file");
    let mut lexer = lexer::Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexer error");
    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse_program().expect("Parser error");

    println!("{:#?}", program);
}
