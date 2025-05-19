use crate::visitor::Visitor;
use crate::visitor::Visitable;
use crate::ast;
use crate::tokens;

pub struct AstPrinterVisitor {
    pub indent: usize,
}

impl AstPrinterVisitor {
    pub fn new() -> Self {
        AstPrinterVisitor { indent: 0 }
    }
    fn pad(&self) -> String {
        "  ".repeat(self.indent)
    }
}

impl Visitor for AstPrinterVisitor {
    fn visit_program(&mut self, program: &ast::Program) {
        println!("{}Program", self.pad());
        self.indent += 1;
        program.expression_list.accept(self);
        self.indent -= 1;
    }
    fn visit_expression_list(&mut self, expr_list: &ast::ExpressionList) {
        println!("{}ExpressionList", self.pad());
        self.indent += 1;
        for expr in &expr_list.expressions {
            expr.accept(self);
        }
        self.indent -= 1;
    }
    fn visit_expression(&mut self, expr: &ast::Expression) {
        match expr {
            ast::Expression::BinaryOp(binop) => binop.accept(self),
            ast::Expression::Atom(atom) => atom.accept(self),
        }
    }
    fn visit_atom(&mut self, atom: &ast::atoms::atom::Atom) {
        use crate::ast::atoms::atom::Atom::*;
        match atom {
            LetIn(letin) => letin.accept(self),
            Block(block) => block.accept(self),
            Group(expr) => {
                println!("{}Group", self.pad());
                expr.accept(self);
            }
            NumberLiteral(lit) => lit.accept(self),
            BooleanLiteral(lit) => lit.accept(self),
            StringLiteral(lit) => lit.accept(self),
            Variable(id) => id.accept(self),
        }
    }
    fn visit_binary_op(&mut self, binop: &ast::expressions::binoperation::BinaryOp) {
        println!("{}BinaryOp: {}", self.pad(), binop.operator);
        self.indent += 1;
        binop.left.accept(self);
        binop.right.accept(self);
        self.indent -= 1;
    }
    fn visit_letin(&mut self, letin: &ast::atoms::letin::LetIn) {
        println!("{}LetIn", self.pad());
        self.indent += 1;
        for assign in &letin.bindings {
            assign.accept(self);
        }
        letin.body.accept(self);
        self.indent -= 1;
    }
    fn visit_assignment(&mut self, assign: &ast::atoms::letin::Assignment) {
        println!("{}Assignment: {} {}", self.pad(), assign.identifier, assign.op);
        self.indent += 1;
        assign.body.accept(self);
        self.indent -= 1;
    }
    fn visit_block(&mut self, block: &ast::atoms::block::Block) {
        println!("{}Block", self.pad());
        self.indent += 1;
        block.expression_list.accept(self);
        self.indent -= 1;
    }
    fn visit_literal(&mut self, literal: &tokens::Literal) {
        println!("{}Literal: {}", self.pad(), literal);
    }
    fn visit_identifier(&mut self, identifier: &tokens::Identifier) {
        println!("{}Identifier: {}", self.pad(), identifier);
    }
}