#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Print(Box<Expr>),
    LetIn {
        var: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub expressions: Vec<Expr>,
}
