use super::super::{Visitable, Visitor};
use crate::Expression;
use crate::tokens::{Identifier };

#[derive(Debug)]
pub struct FunctionParams {
    pub name: Identifier,
    pub signature: String,
}

impl FunctionParams {
    pub fn new(name: Identifier, signature: String) -> Self {
        FunctionParams {
            name,
            signature,
        }
    }
}



#[derive(Debug)]
pub struct FunctionDef {
    pub name: Identifier,
    pub params: Vec<FunctionParams>,
    pub return_type: String,
    pub body: Box<Expression>,
}

impl FunctionDef {
    pub fn new_expr(name: Identifier, params: Vec<FunctionParams>, return_type: String, expr: Box<Expression>) -> Self {
        FunctionDef {
            name,
            params,
            return_type,
            body: expr,
        }
    }
}

impl Visitable for FunctionDef {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_function_def(self);
    }
}