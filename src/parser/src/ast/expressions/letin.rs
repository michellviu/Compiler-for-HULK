use super::super::Visitable;
use super::super::Visitor;
use crate::Atom;
use crate::BinOp;
use crate::Expression;
use crate::tokens::*;

#[derive(Debug)]
pub struct Assignment {
    pub variable: Atom,
    pub op: BinOp,
    pub body: Box<Expression>,
}

impl Assignment {
    pub fn new(variable: Atom, op: BinOp, body: Expression) -> Self {
        match variable {
            
            Atom::Identifier(identifier) => Assignment {
                variable: Atom::Identifier(identifier),
                op,
                body: Box::new(body),
            },
            _ => panic!("Assignment must be to a variable"),
        }
    }
}

impl Visitable for Assignment {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_assignment(self);
    }
}
#[derive(Debug)]
pub struct LetIn {
    pub let_token: Keyword,
    pub bindings: Vec<Assignment>,
    pub in_token: Keyword,
    pub body: Box<Expression>,
}

impl LetIn {
    pub fn new(
        let_token: Keyword,
        bindings: Vec<Assignment>,
        in_token: Keyword,
        body: Expression,
    ) -> Self {
        LetIn {
            let_token,
            bindings,
            in_token,
            body: Box::new(body),
        }
    }
}

impl Visitable for LetIn {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_letin(self);
    }
}
