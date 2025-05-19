use parser::grammar::ProgramParser;
use parser::visitor::AstPrinterVisitor::AstPrinterVisitor;
use parser::visitor::Visitable;

fn main() {
    let expr = ProgramParser::new()
        .parse("let x = let z=4 in (z*4),y=6 in let j=20 in (j+x+y)")
        .unwrap();
    let mut printer = AstPrinterVisitor::new();
    expr.accept(&mut printer);
}


// mod ast;
// mod error;
// mod lexer;
// mod parser;

// use std::env;
// use std::fs;

//     let source = fs::read_to_string(&args[1]).expect("Failed to read file");
//     let mut lexer = lexer::Lexer::new(&source);
//     let tokens = lexer.tokenize().expect("Lexer error");
//     let mut parser = parser::Parser::new(tokens);
//     let program = parser.parse_program().expect("Parser error");

//     println!("{:#?}", program);
// }
