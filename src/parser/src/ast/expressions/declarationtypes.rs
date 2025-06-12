use super::super::Visitable;
use super::super::Visitor;
mod letin;
use letin::Assignment;
use crate::tokens::{Identifier };
mod functiondeclaration;
use functiondeclaration::{FunctionDef, Params};


#[derive(Debug)]
pub struct Declarationtypes {
    pub name_types: Identifier,
    pub properties: Vec<Assignment>,
    pub functions: Vec<FunctionDef>,
    pub build: Params

}

impl Declarationtypes {
    pub fn new_expr(name_types: Identifier, properties: Vec<Assignment>, functions: Vec<FunctionDef>, build: Params ) -> Self {
        Declarationtypes {
            name_types,
            properties,
            functions,
            build: expr,
        }
    }
}

impl Visitable for Declarationtypes {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_declaration_function(self);
    }
}