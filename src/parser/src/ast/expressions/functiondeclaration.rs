use super::super::{Visitable, Visitor,Type};
use crate::Expression;
use crate::tokens::{Identifier };


#[derive(Debug,Clone)]
pub struct FunctionParams {
    pub name: Identifier,
    pub signature: Type,
}

impl FunctionParams {
    pub fn new(name: Identifier, signature: Type) -> Self {
        FunctionParams {
            name,
            signature,
        }
    }
}



#[derive(Debug,Clone)]
pub struct FunctionDef {
    pub name: Identifier,
    pub params: Vec<FunctionParams>,
    pub return_type: Type,
    pub body: Box<Expression>,
}

impl FunctionDef {
    pub fn new_expr(name: Identifier, params: Vec<FunctionParams>, return_type: Type, expr: Box<Expression>) -> Self {
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