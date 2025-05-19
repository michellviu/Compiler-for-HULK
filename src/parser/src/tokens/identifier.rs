use std::fmt;
use super::position::Position;
use super::super::Visitable;
use super::super::Visitor;
#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub position: Position,
}

impl Identifier {
    pub fn new(start: usize, end: usize, id: &str) -> Self {
        Identifier {
            position: Position::new(start, end),
            name: id.to_string(),
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Visitable for Identifier {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_identifier(&self);
    }
    
}