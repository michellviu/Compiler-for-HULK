use super::super::Visitable;
use super::super::Visitor;
use super::*;
use crate::Atom;
use crate::BinOp;
use crate::tokens;

#[derive(Debug)]
pub enum Expression {
    BinaryOp(BinaryOp),
    Atom(Box<Atom>),
    IfElse(Box<ifelse::IfElse>),
    LetIn(Box<letin::LetIn>), // ← esta línea nueva
    Print(Box<Expression>, tokens::Position), // Nueva variante
    While(Box<Expression>, Box<Expression>),  // ← esta línea nueva
}

impl Expression {
    pub fn new_ifelse(ifelse: ifelse::IfElse) -> Self {
        Expression::IfElse(Box::new(ifelse))
    }

    pub fn new_binary_op(left: Expression, right: Expression, operator: BinOp) -> Self {
        Expression::BinaryOp(BinaryOp::new(left, right, operator))
    }

    pub fn new_atom(atom: Atom) -> Self {
        Expression::Atom(Box::new(atom))
    }

    pub fn new_print(expr: Expression, pos: tokens::Position) -> Self {
        Expression::Print(Box::new(expr), pos)
    }

    pub fn new_while(cond: Expression, body: Expression) -> Self {
        Expression::While(Box::new(cond), Box::new(body))
    }

    pub fn new_letin(letin: letin::LetIn) -> Self {
        Expression::LetIn(Box::new(letin))
    }
}

impl Visitable for Expression {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Expression::BinaryOp(binop) => visitor.visit_binary_op(binop),
            Expression::Atom(atom) => atom.accept(visitor),
            Expression::IfElse(ifelse) => ifelse.accept(visitor),
            Expression::Print(expr, pos) => visitor.visit_print(expr),
            Expression::While(cond, body) => visitor.visit_while(cond, body), // ← nuevo
            Expression::LetIn(letin) => letin.accept(visitor), // ← nueva variante
        }
    }
}
