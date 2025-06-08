use crate::tokens::Position;
use crate::ast::Program;
use crate::grammar::{ProgramParser, Token};
use lalrpop_util::ParseError as LalrpopError;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub position: Option<Position>,
}

impl ParseError {
    pub fn new<S: Into<String>>(message: S, position: Option<Position>) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }
}

/// Punto de entrada del parser con manejo de errores
pub fn parse_program(input: &str) -> Result<Program, ParseError> {
    let parser = ProgramParser::new();
    match parser.parse(input) {
        Ok(program) => Ok(program),
        Err(err) => Err(map_lalrpop_error(err)),
    }
}

fn map_lalrpop_error(err: LalrpopError<usize, Token, &str>) -> ParseError {
    use lalrpop_util::ParseError::*;

    match err {
        InvalidToken { location } => {
            ParseError::new("Token inválido", Some(Position::new(location, location)))
        }
        UnrecognizedToken { token: (start, _, end), expected } => {
            ParseError::new(
                format!("Token no reconocido, se esperaba uno de: {:?}", expected),
                Some(Position::new(start, end)),
            )
        }
        ExtraToken { token: (start, _, end) } => {
            ParseError::new("Token extra", Some(Position::new(start, end)))
        }
        UnrecognizedEof { location, expected } => {
            ParseError::new(
                format!("EOF inesperado, se esperaba: {:?}", expected),
                Some(Position::new(location, location)),
            )
        }
        User { error } => {
            ParseError::new(error, None)
        }
    }
}
