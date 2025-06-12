use super::super::{Visitable, Visitor};
use crate::Expression;
use crate::tokens::{Identifier };


#[derive(Debug)]
pub struct FunctionParams {
    pub name: Atom,
    pub signature: String,
}

impl FunctionParams {
    pub fn new(name: Atom, signature: String) -> Self {
        FunctionParams { name, signature }
    }
}




#[derive(Debug)]
pub struct FunctionDef {
    pub function_kw: Keyword,
    pub name: Atom,
    pub params: Vec<FunctionParams>,
    // pub return_type: Atom,
    pub body: Box<Expression>,
    pub is_inline: bool,
}

impl FunctionDef {
    pub fn new(
        function_kw: Keyword,
        name: Atom,
        params: Vec<FunctionParams>,
        // return_type: Atom,
        expr: Box<Expression>,
        is_inline: bool,
    ) -> Self {
        FunctionDef {
            function_kw,
            name,
            params,
            // return_type,
            body: expr,
            is_inline,
        }
    }
}

impl Visitable for FunctionDef {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_function_def(self);
    }
}
