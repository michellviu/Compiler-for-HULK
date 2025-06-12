use super::super::Expression;
use crate::tokens::GroupingOperator;
use super::super::Visitable;
use super::super::Visitor;

#[derive(Debug,Clone)]
pub struct ExpressionList {
    pub expressions: Vec<Expression>,
}

impl ExpressionList {
    pub fn new(expressions: Vec<Expression>) -> Self {
        ExpressionList {
            expressions,
        }
    }
}

impl Visitable for ExpressionList {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_expression_list(self);
    }
    
}
#[derive(Debug,Clone)]
pub struct Block {
    pub open_brace: GroupingOperator,
    pub close_brace: GroupingOperator,
    pub expression_list: ExpressionList,
}

impl Block {
    pub fn new(
        open_brace: GroupingOperator,
        expression_list: ExpressionList,
        close_brace: GroupingOperator,
    ) -> Self {
        Block {
            open_brace,
            close_brace,
            expression_list,
        }
    }
}

impl Visitable for Block {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_block(self);
    }
    
}