use super::super::Visitable;
use super::super::Visitor;
use crate::tokens::{Identifier };
use crate::Expression;

#[derive(Debug)]
pub struct InstantingTypes {
    pub referenced_type: Identifier,
    pub params: Vec<Expression>,

}

impl InstantingTypes {
    pub fn new_expr(referenced_type: Identifier, params: Vec<Expression> ) -> Self {
        InstantingTypes {
            referenced_type,
            params,
        }
    }
}

impl Visitable for InstantingTypes {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_instanting_types(self);
    }
}