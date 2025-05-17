use super::BinOp;
use std::fmt;

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, BinOp, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Op(left, op, right) => write!(f, "({} {} {})", left, op, right),
        }
    }
}