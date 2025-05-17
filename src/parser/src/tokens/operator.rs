use std::fmt;

pub enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Add => "+",
            BinOp::Sub => "-",
        };
        write!(f, "{}", s)
    }
}

pub enum GroupingOperator {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
}

impl fmt::Display for GroupingOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GroupingOperator::OpenParen => "(",
            GroupingOperator::CloseParen => ")",
            GroupingOperator::OpenBrace => "{",
            GroupingOperator::CloseBrace => "}",
        };
        write!(f, "{}", s)
    }
}
