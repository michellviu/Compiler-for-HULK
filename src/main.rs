use  parser::grammar::ExprParser;

#[test]
fn calculator4() {
    let expr = ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{}", expr), "((22 * 44) + 66)");
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
