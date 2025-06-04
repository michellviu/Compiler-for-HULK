use super::super::{Visitable, Visitor};
use crate::Expression;
use crate::tokens::Keyword;

#[derive(Debug)]
pub struct IfElse {
    pub if_kw: Keyword,
    pub condition: Box<Expression>,
    pub then_branch: Box<Expression>,
    pub elif_branches: Vec<(Keyword, Expression, Expression)>,
    pub else_kw: Option<Keyword>,
    pub else_branch: Option<Box<Expression>>,
}

impl IfElse {
    pub fn new(
        if_kw: Keyword,
        condition: Expression,
        then_branch: Expression,
        elif_branches: Vec<(Keyword, Expression, Expression)>,
        else_kw: Option<Keyword>,
        else_branch: Option<Expression>,
    ) -> Self {
        IfElse {
            if_kw,
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            elif_branches,
            else_kw,
            else_branch: else_branch.map(Box::new),
        }
    }
}

impl Visitable for IfElse {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_ifelse(self);
    }
}
