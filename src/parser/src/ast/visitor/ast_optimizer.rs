use crate::ast;
use crate::ast::visitor::{Visitor, Visitable};
use crate::ast::expressions::binoperation::BinaryOp;
use crate::tokens::BinOp;

pub struct AstOptimizer;

impl AstOptimizer {
    pub fn new() -> Self {
        AstOptimizer
    }

    fn merge_string_literals(&self, binop: &BinaryOp) -> Option<ast::Expression> {
        if let BinOp::ConcatString(_) = binop.operator {
            if let ast::Expression::Atom(left_atom) = &*binop.left {
                if let ast::Expression::Atom(right_atom) = &*binop.right {
                    if let ast::atoms::atom::Atom::StringLiteral(left_lit) = &**left_atom {
                        if let ast::atoms::atom::Atom::StringLiteral(right_lit) = &**right_atom {
                            // Merge the two string literals
                            let merged_value = format!(
                                "{}{}",
                                left_lit.to_string_value(),
                                right_lit.to_string_value()
                            );
                            return Some(ast::Expression::Atom(Box::new(
                                ast::atoms::atom::Atom::new_string_literal(
                                    left_lit.position().start(),
                                    right_lit.position().end(),
                                    &merged_value,
                                ),
                            )));
                        }
                    }
                }
            }
        }
        None
    }
}

impl Visitor for AstOptimizer {
    fn visit_program(&mut self, program: &ast::Program) {
        for expr in &program.expressions {
            expr.accept(self);
        }
    }

    fn visit_expression_list(&mut self, expr_list: &ast::ExpressionList) {
        for expr in &expr_list.expressions {
            expr.accept(self);
        }
    }

    fn visit_expression(&mut self, expr: &ast::Expression) {
        expr.accept(self);
    }

    fn visit_atom(&mut self, _atom: &ast::atoms::atom::Atom) {
        // Nothing to optimize in atoms
    }

    fn visit_binary_op(&mut self, binop: &BinaryOp) {
        binop.left.accept(self);
        binop.right.accept(self);

        if let Some(merged_expr) = self.merge_string_literals(binop) {
            // Here we would replace the current node with merged_expr in the AST
            // But since we have immutable references, this requires a mutable AST or a transformation pass
            // For now, this is a placeholder to indicate where merging would happen
        }
    }

    fn visit_letin(&mut self, letin: &ast::atoms::letin::LetIn) {
        for assign in &letin.assignments {
            assign.accept(self);
        }
        letin.expression.accept(self);
    }

    fn visit_assignment(&mut self, assign: &ast::atoms::letin::Assignment) {
        assign.expression.accept(self);
    }

    fn visit_block(&mut self, block: &ast::atoms::block::Block) {
        for expr in &block.expressions.expressions {
            expr.accept(self);
        }
    }

    fn visit_literal(&mut self, _literal: &crate::tokens::Literal) {}

    fn visit_identifier(&mut self, _identifier: &crate::tokens::Identifier) {}

    fn visit_print(&mut self, expr: &ast::Expression, _pos: &crate::tokens::Position) {
        expr.accept(self);
    }
}
