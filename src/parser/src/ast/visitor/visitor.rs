use crate::ast;
use crate::tokens;
pub trait Visitor {
    fn visit_program(&mut self, program: &ast::Program);
    fn visit_expression_list(&mut self, expr_list: &ast::ExpressionList);
    fn visit_expression(&mut self, expr: &ast::Expression);
    fn visit_atom(&mut self, atom: &ast::atoms::atom::Atom);
    fn visit_binary_op(&mut self, binop: &ast::expressions::binoperation::BinaryOp);
    fn visit_letin(&mut self, letin: &ast::atoms::letin::LetIn);
    fn visit_assignment(&mut self, assign: &ast::atoms::letin::Assignment);
    fn visit_block(&mut self, block: &ast::atoms::block::Block);
    fn visit_literal(&mut self, literal: &tokens::Literal);
    fn visit_identifier(&mut self, identifier: &tokens::Identifier);
    fn visit_print(&mut self, expr: &ast::Expression, pos: &tokens::Position);
    fn visit_while(&mut self, cond: &ast::Expression, body: &ast::Expression);
}

pub trait Visitable {
    fn accept<V: Visitor>(&self, visitor: &mut V);
}