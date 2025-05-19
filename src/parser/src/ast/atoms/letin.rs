use crate::tokens::*;
use crate::BinOp;
use crate::Expression;
use crate::Atom;

#[derive(Debug)]
pub struct Assignment {
    pub identifier: Identifier,
    pub op: BinOp,
    pub body: Box<Expression>,
}

impl Assignment {
    pub fn new(identifier: Identifier, op: BinOp, body: Expression) -> Self {
        Assignment {
            identifier,
            op,
            body: Box::new(body),
        }
    }
}
#[derive(Debug)]
pub struct LetIn {
    pub let_token: Keyword,
    pub bindings: Vec<Assignment>,
    pub in_token: Keyword,
    pub body: Box<Atom>,
}

impl LetIn {
    
    pub fn new(let_token:Keyword, bindings: Vec<Assignment>,in_token:Keyword, body: Atom) -> Self {
        LetIn {
            let_token,
            bindings,
            in_token,
            body: Box::new(body),
        }
    }
}