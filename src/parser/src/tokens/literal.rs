use std::fmt;
use super::position::Position;

#[derive(Debug)]
pub enum Literal {
    Number(i32, Position),
    Str(String, Position),
    Bool(bool, Position),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n, _) => write!(f, "{}", n),
            Literal::Str(s, _) => write!(f, "\"{}\"", s),
            Literal::Bool(b, _) => write!(f, "{}", b),
        }
    }
}