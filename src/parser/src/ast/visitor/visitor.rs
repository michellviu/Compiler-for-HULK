use crate::ast;
use crate::tokens;
use crate::whilee;
pub trait Visitor {
    fn visit_program(&mut self, program: &ast::Program);
    fn visit_expression_list(&mut self, expr_list: &ast::ExpressionList);
    fn visit_expression(&mut self, expr: &ast::Expression);
    fn visit_atom(&mut self, atom: &ast::atoms::atom::Atom);
    fn visit_binary_op(&mut self, binop: &ast::expressions::binoperation::BinaryOp);
    fn visit_letin(&mut self, letin: &ast::expressions::letin::LetIn);
    fn visit_assignment(&mut self, assign: &ast::expressions::letin::Assignment);
    fn visit_block(&mut self, block: &ast::expressions::block::Block);
    fn visit_literal(&mut self, literal: &tokens::Literal);
    fn visit_identifier(&mut self, identifier: &tokens::Identifier);
    fn visit_print(&mut self, expr: &ast::Expression);
    fn visit_while(&mut self, whilee: &whilee::While);
    fn visit_ifelse(&mut self, ifelse: &ast::expressions::ifelse::IfElse);
    fn visit_group(&mut self, group: &ast::atoms::group::Group);
    fn visit_unary_op(&mut self, unary_op: &ast::expressions::unaryoperation::UnaryOp);
    fn visit_functdef(&mut self, functdef: &ast::expressions::functiondeclaration::FunctionDef);
    fn visit_functcall(&mut self, functcall: &ast::expressions::functioncall::FunctionCall);
}

pub trait Visitable {
    fn accept<V: Visitor>(&self, visitor: &mut V);
}
