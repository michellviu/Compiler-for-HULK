use super::super::Visitable;
use super::super::Visitor;

use crate::tokens::{Identifier };

use super::functiondeclaration::{FunctionDef, FunctionParams};
use super::letin::Assignment;


#[derive(Debug,Clone)]
pub struct Declarationtypes {
    pub name_types: Identifier,
    pub properties: Vec<Assignment>,
    pub functions: Vec<FunctionDef>,
    pub build: Vec<FunctionParams>

}

impl Declarationtypes {
    pub fn new_expr(name_types: Identifier, properties: Vec<Assignment>, functions: Vec<FunctionDef>, build: Vec<FunctionParams> ) -> Self {
        Declarationtypes {
            name_types,
            properties,
            functions,
            build,
        }
    }
}

impl Visitable for Declarationtypes {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_type_declaration(self);
    }
}