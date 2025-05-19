use super::Expression;
use crate::tokens::BinOp;

#[derive(Debug)]
pub struct BinaryOp{
    left: Box<Expression>,
    right: Box<Expression>,
    operator: BinOp,
}

impl BinaryOp {
    pub fn new(left: Expression, right: Expression, operator:BinOp) -> Self {
        BinaryOp {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        }
    }
}