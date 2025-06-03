use crate::ast::Atom;
use crate::ast::Expression;
use crate::tokens::Keyword;
use super::super::Visitor;
use super::super::Visitable;

#[derive(Debug)]
pub struct IfElse {
    pub if_token: Keyword,
    pub condition: Box<Expression>,
    pub then_branch: Box<Atom>,
    pub elif_branches: Vec<(Keyword, Expression, Atom)>, // (elif token, cond, branch)
    pub else_token: Keyword,
    pub else_branch: Box<Atom>,
}

impl IfElse {
    pub fn new(
        if_token: Keyword,
        condition: Box<Expression>,
        then_branch: Box<Atom>,
        elif_branches: Vec<(Keyword, Expression, Atom)>,
        else_token: Keyword,
        else_branch: Box<Atom>,
    ) -> Self {
        Self {
            if_token,
            condition: condition,
            then_branch: then_branch,
            elif_branches,
            else_token,
            else_branch: else_branch,
        }
    }
}

impl Visitable for IfElse {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_ifelse(self);
    }
}

