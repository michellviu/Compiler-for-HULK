use super::ExpressionList;
use super::Visitable;
use super::Visitor;

#[derive(Debug)]
pub struct Program {
    pub expression_list: ExpressionList
}

impl Program {
    pub fn new(expression_list: ExpressionList) -> Self {
        Program {
            expression_list
        }
    }
}

impl Visitable for Program {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_program(self);
    }
    
}