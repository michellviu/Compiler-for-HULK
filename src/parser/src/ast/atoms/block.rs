use super::super::Expression;
use crate::tokens::GroupingOperator;

#[derive(Debug)]
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
#[derive(Debug)]
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