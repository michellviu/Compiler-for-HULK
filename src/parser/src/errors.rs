use crate::tokens::Position;
use crate::ast::Program;
use crate::grammar::{ProgramParser, Token};
use lalrpop_util::ParseError as LalrpopError;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: Option<usize>, // Solo la línea donde ocurre el error
}

impl ParseError {
    pub fn new<S: Into<String>>(message: S, line: Option<usize>) -> Self {
        Self {
            message: message.into(),
            line,
        }
    }
}

/// Punto de entrada del parser con manejo de errores
pub fn parse_program(input: &str) -> Result<Program, ParseError> {
    let parser = ProgramParser::new();
    match parser.parse(input) {
        Ok(program) => Ok(program),
        Err(err) => Err(map_lalrpop_error(err, input)),
    }
}

fn map_lalrpop_error(err: LalrpopError<usize, Token, &str>, input: &str) -> ParseError {
    use lalrpop_util::ParseError::*;

    // Función auxiliar para ajustar la línea si la posición apunta al inicio de una línea
    fn adjusted_line(pos: Position, input: &str) -> usize {
        let line = pos.start_line(input);
        // Si la posición start es mayor que 0 y el carácter anterior es '\n', restar 1
        if pos.start > 0 {
            if let Some(prev_char) = input.chars().nth(pos.start - 1) {
                if prev_char == '\n' {
                    return line.saturating_sub(1);
                }
            }
        }
        line
    }

    match err {
        InvalidToken { location } => {
            let pos = Position::new(location, location);
            ParseError::new("Token inválido", Some(adjusted_line(pos, input)))
        }
        UnrecognizedToken { token: (start, _, end), expected } => {
            let pos = Position::new(start, end);
            ParseError::new(
                format!("Token no reconocido, se esperaba uno de: {:?}", expected),
                Some(adjusted_line(pos, input)),
            )
        }
        ExtraToken { token: (start, _, end) } => {
            let pos = Position::new(start, end);
            ParseError::new("Token extra", Some(adjusted_line(pos, input)))
        }
        UnrecognizedEof { location, expected } => {
            let pos = Position::new(location, location);
            ParseError::new(
                format!("EOF inesperado, se esperaba: {:?}", expected),
                Some(adjusted_line(pos, input)),
            )
        }
        User { error } => {
            ParseError::new(error, None)
        }
    }
}
