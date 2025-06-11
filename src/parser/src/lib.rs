use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);

mod ast;
pub use ast::*;

pub mod tokens;
pub use tokens::*;

pub mod errors;
pub use errors::{ParseError, parse_program};

pub use grammar::ProgramParser;

pub use ast::expressions::functioncall;
pub use ast::expressions::functiondeclaration;