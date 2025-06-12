use crate::tokens::*;
use crate::Expression;
use super::super::Visitable;
use super::super::Visitor;

#[derive(Debug,Clone)]
pub struct For {
    pub for_token: Keyword,
    pub var: Box<Expression>,
    pub in_token: Keyword,
    pub iterable: Box<Expression>,
    pub body: Box<Expression>,
}

impl For {
    pub fn new(for_token: Keyword, var: Expression, in_token: Keyword, iterable: Expression, body: Expression) -> Self {
        For {
            for_token,
            var: Box::new(var),
            in_token,
            iterable: Box::new(iterable),
            body: Box::new(body),
        }
    }
}

impl Visitable for For {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_for(self);
    }
}