use super::super::Visitable;
use super::super::Visitor;
use super::*;
use crate::Atom;
use crate::BinOp;
use crate::tokens;
use super::declarationtypes::Declarationtypes;
use super::instantiatingtypes::InstantingTypes;
use super::accesstypesprop::AccessTypeProp;


#[derive(Debug,Clone)]
pub enum Expression {
    BinaryOp(BinaryOp),
    Atom(Box<Atom>),
    IfElse(Box<ifelse::IfElse>),
    LetIn(Box<letin::LetIn>),
    For(Box<forr::For>),
    Print(Box<Expression>, tokens::Position),
    While(Box<whilee::While>),
    Block(Box<block::Block>),
    UnaryOp(UnaryOp),
    Range(Box<Expression>, Box<Expression>),
    FunctionCall(functioncall::FunctionCall),
    FunctionDef(functiondeclaration::FunctionDef),
    TypeDeclaration(Box<Declarationtypes>),
    TypeInstantiation(Box<InstantingTypes>),
    TypeMethodAccess(Box<AccessTypeProp>),
    TypePropertyAccess(Box<accesstypesprop::AccessTypeProp>),
    
}

impl Expression {
    pub fn new_type_property_access(acc: AccessTypeProp) -> Self {
        Expression::TypePropertyAccess(Box::new(acc))
    }
    pub fn new_type_declaration(decl: Declarationtypes) -> Self {
        Expression::TypeDeclaration(Box::new(decl))
    }
    pub fn new_type_instantiation(inst: InstantingTypes) -> Self {
        Expression::TypeInstantiation(Box::new(inst))
    }
    pub fn new_type_method_access(acc: AccessTypeProp) -> Self {
        Expression::TypeMethodAccess(Box::new(acc))
    }
    pub fn new_range(start: Expression, end: Expression) -> Self {
        Expression::Range(Box::new(start), Box::new(end))
    }
    pub fn new_for(forr: forr::For) -> Self {
        Expression::For(Box::new(forr))
    }
    pub fn new_ifelse(ifelse: ifelse::IfElse) -> Self {
        Expression::IfElse(Box::new(ifelse))
    }

    pub fn new_binary_op(left: Expression, right: Expression, operator: BinOp) -> Self {
        Expression::BinaryOp(BinaryOp::new(left, right, operator))
    }

    pub fn new_unary_op(op: tokens::UnaryOp, expr: Expression) -> Self
    {
        Expression::UnaryOp(UnaryOp::new(op, expr))
    }

    pub fn new_atom(atom: Atom) -> Self {
        Expression::Atom(Box::new(atom))
    }

    pub fn new_print(expr: Expression, pos: tokens::Position) -> Self {
        Expression::Print(Box::new(expr), pos)
    }

    pub fn new_while(whilee: While) -> Self {
        Expression::While(Box::new(whilee))
    }

    pub fn new_letin(letin: letin::LetIn) -> Self {
        Expression::LetIn(Box::new(letin))
    }

    pub fn new_block(block: Block) -> Self {
        Expression::Block(Box::new(block))
    }


}

impl Visitable for Expression {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Expression::BinaryOp(binop) => visitor.visit_binary_op(binop),
            Expression::Atom(atom) => atom.accept(visitor),
            Expression::IfElse(ifelse) => ifelse.accept(visitor),
            Expression::Print(expr, _pos) => visitor.visit_print(expr),
            Expression::While(whilee) => whilee.accept(visitor),
            Expression::LetIn(letin) => letin.accept(visitor),
            Expression::Block(block) => block.accept(visitor),
            Expression::UnaryOp(unoperator) => unoperator.accept(visitor),
            Expression::For(forr) => forr.accept(visitor),
            Expression::Range(start, end) => {
                start.accept(visitor);
                end.accept(visitor);
            }
            Expression::FunctionCall(call) => call.accept(visitor),
            Expression::FunctionDef(def) => def.accept(visitor),
            Expression::TypeDeclaration(decl) => decl.accept(visitor),
            Expression::TypeInstantiation(inst) => inst.accept(visitor),
            Expression::TypeMethodAccess(acc) => acc.accept(visitor),
            Expression::TypePropertyAccess(acc) => acc.accept(visitor),
        }
    }
}
