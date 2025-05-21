use super::*;
use crate::Atom;
use crate::BinOp;
use crate::tokens;
use super::super::Visitor;
use super::super::Visitable;

#[derive(Debug)]
pub enum Expression {
    BinaryOp(BinaryOp),
    Atom(Box<Atom>),
    Print(Box<Expression>, tokens::Position), // Nueva variante
}

impl Expression {
    pub fn new_binary_op(left: Expression, right: Expression, operator: BinOp) -> Self {
        Expression::BinaryOp(BinaryOp::new(left, right, operator))
    }

    pub fn new_atom(atom: Atom) -> Self {
        Expression::Atom(Box::new(atom))
    }

    pub fn new_print(expr: Expression, pos: tokens::Position) -> Self {
        Expression::Print(Box::new(expr), pos)
    }
}

impl Visitable for Expression {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_expression(&self);
    }
}