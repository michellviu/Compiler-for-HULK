use crate ::tokens::*;
use super::super::Expression;
use super::super::Visitor;
use super::super::Visitable;

#[derive(Debug)]
pub struct Print {
    pub print_token: Keyword,
    pub expression: Box<Expression>,
}

impl Print {
    pub fn new(print_token: Keyword, expression: Expression) -> Self {
        Print {
            print_token,
            expression: Box::new(expression),
        }
    }
}

impl Visitable for Print {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_print(self);
    }
}