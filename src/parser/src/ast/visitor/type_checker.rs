use crate::ast;
use crate::tokens;
use crate::ast::visitor::{Visitor, Visitable};
use crate::ast::expressions::binoperation::BinaryOp;
use crate::tokens::BinOp;

pub struct TypeChecker {
    pub errors: Vec<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker { errors: Vec::new() }
    }

    fn is_string_type(&self, expr: &ast::Expression) -> bool {
        match expr {
            ast::Expression::Atom(atom) => match &**atom {
                ast::atoms::atom::Atom::StringLiteral(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn is_number_type(&self, expr: &ast::Expression) -> bool {
        match expr {
            ast::Expression::Atom(atom) => match &**atom {
                ast::atoms::atom::Atom::NumberLiteral(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn coerce_to_string(&self, expr: &ast::Expression) -> ast::Expression {
        // For simplicity, wrap the expression in a function call to string conversion
        // This requires a String conversion function in the runtime, e.g. to_string()
        // Here we just return the expression as is, assuming implicit coercion is handled elsewhere
        expr.clone()
    }
}

impl Visitor for TypeChecker {
    fn visit_program(&mut self, program: &ast::Program) {
        program.expressions.iter().for_each(|e| e.accept(self));
    }

    fn visit_expression_list(&mut self, expr_list: &ast::ExpressionList) {
        expr_list.expressions.iter().for_each(|e| e.accept(self));
    }

    fn visit_expression(&mut self, expr: &ast::Expression) {
        expr.accept(self);
    }

    fn visit_atom(&mut self, atom: &ast::atoms::atom::Atom) {
        // No type checking needed for atoms here
    }

    fn visit_binary_op(&mut self, binop: &BinaryOp) {
        binop.left.accept(self);
        binop.right.accept(self);

        if let BinOp::ConcatString(_) = binop.operator {
            let left_is_string = self.is_string_type(&binop.left);
            let right_is_string = self.is_string_type(&binop.right);
            let left_is_number = self.is_number_type(&binop.left);
            let right_is_number = self.is_number_type(&binop.right);

            if !(left_is_string || left_is_number) {
                self.errors.push(format!("Left operand of @ must be string or number"));
            }
            if !(right_is_string || right_is_number) {
                self.errors.push(format!("Right operand of @ must be string or number"));
            }

            // Coercion logic would go here if we mutate the AST
            // For now, just check types and report errors
        }
    }

    fn visit_letin(&mut self, letin: &ast::atoms::letin::LetIn) {
        letin.assignments.iter().for_each(|a| a.accept(self));
        letin.expression.accept(self);
    }

    fn visit_assignment(&mut self, assign: &ast::atoms::letin::Assignment) {
        assign.expression.accept(self);
    }

    fn visit_block(&mut self, block: &ast::atoms::block::Block) {
        block.expressions.expressions.iter().for_each(|e| e.accept(self));
    }

    fn visit_literal(&mut self, _literal: &tokens::Literal) {}

    fn visit_identifier(&mut self, _identifier: &tokens::Identifier) {}

    fn visit_print(&mut self, expr: &ast::Expression, _pos: &tokens::Position) {
        expr.accept(self);
    }
}
