use super::ExpressionList;

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