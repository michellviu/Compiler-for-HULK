use super::super::{Visitable, Visitor};
use crate::Expression;
use crate::tokens::Identifier;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub funct_name: Identifier,             
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    pub fn new(funct_name: Identifier, arguments: Vec<Expression>) -> Self {
        FunctionCall { funct_name, arguments }
    }
}

impl Visitable for FunctionCall {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_functcall(self);
    }
}