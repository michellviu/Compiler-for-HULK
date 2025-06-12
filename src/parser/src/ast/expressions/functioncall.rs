use super::super::{Visitable, Visitor};
use crate::{Atom, Expression};

#[derive(Debug)]
pub struct FunctionCall {
    pub funct_name: Atom,             
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    pub fn new(funct_name: Atom, arguments: Vec<Expression>) -> Self {
        FunctionCall { funct_name, arguments }
    }
}

impl Visitable for FunctionCall {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_function_call(self);
    }
}