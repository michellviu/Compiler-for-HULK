use super::super::*;
// use super::letin::Assignment;
use crate::tokens::*;
#[derive(Debug)]
pub enum Atom{

    NumberLiteral(Literal),
    BooleanLiteral(Literal),
    StringLiteral(Literal),
    Variable(Identifier),
}


impl Atom {

    pub fn new_number_literal(start: usize, end: usize, value: &str) -> Result<Self, String> {
        match value.parse::<i32>() {
            Ok(num) => Ok(Atom::NumberLiteral(Literal::Number(num, Position::new(start, end)))),
            Err(_) => Err(format!("No se pudo convertir '{}' a número", value)),
        }
    }

    pub fn new_boolean_literal(start: usize, end: usize, value: &str) -> Result<Self, String> {
        match value.parse::<bool>() {
            Ok(b) => Ok(Atom::BooleanLiteral(Literal::Bool(b, Position::new(start, end)))),
            Err(_) => Err(format!("No se pudo convertir '{}' a booleano", value)),
        }
    }

    pub fn new_string_literal(start: usize, end: usize, value: &str) -> Self {
        Atom::StringLiteral(Literal::Str(value.to_string(), Position::new(start, end)))
    }

    pub fn new_identifier(start: usize, end: usize, id: &str) -> Self {
        Atom::Variable(Identifier {
            name: id.to_string(),
            position: Position::new(start, end),
        })
    }

}


impl Visitable for Atom {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        match self {
            // Atom::LetIn(letin) => visitor.visit_letin(letin),
            // Atom::Block(block) => visitor.visit_block(block),
            // Atom::Group(expr) => visitor.visit_expression(expr),
            Atom::NumberLiteral(literal) => visitor.visit_literal(literal),
            Atom::BooleanLiteral(literal) => visitor.visit_literal(literal),
            Atom::StringLiteral(literal) => visitor.visit_literal(literal),
            Atom::Variable(identifier) => visitor.visit_identifier(identifier),
        }
    }
}