use crate::tokens::*;
use crate::Expression;
use super::super::Visitable;
use super::super::Visitor;

#[derive(Debug,Clone)]
pub struct While {
    pub while_token: Keyword,
    pub cond: Box<Expression>,
    pub body: Box<Expression>,
}

impl While {
    pub fn new(while_token: Keyword, cond: Expression, body: Expression) -> Self {
        While {
            while_token,
            cond: Box::new(cond),
            body: Box::new(body),
        }
    }
}

impl Visitable for While {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_while(self);
    }
}