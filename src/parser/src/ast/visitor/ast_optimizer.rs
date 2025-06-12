use crate::ast;
use crate::ast::expressions::binoperation::BinaryOp;
use crate::ast::visitor::{Visitable, Visitor};

pub struct AstOptimizer;

impl AstOptimizer {
    pub fn new() -> Self {
        AstOptimizer
    }

    // fn merge_string_literals(&self, binop: &BinaryOp) -> Option<ast::Expression> {
    //     if let BinOp::ConcatString(_) = binop.operator {
    //         if let ast::Expression::Atom(left_atom) = &*binop.left {
    //             if let ast::Expression::Atom(right_atom) = &*binop.right {
    //                 if let ast::atoms::atom::Atom::StringLiteral(left_lit) = &**left_atom {
    //                     if let ast::atoms::atom::Atom::StringLiteral(right_lit) = &**right_atom {
    //                         // Merge the two string literals
    //                         let merged_value = format!(
    //                             "{}{}",
    //                             left_lit.to_string(),
    //                             right_lit.to_string()
    //                         );
    //                         return Some(ast::Expression::Atom(Box::new(
    //                             ast::atoms::atom::Atom::new_string_literal(
    //                                 left_lit.,
    //                                 right_lit.position().end(),
    //                                 &merged_value,
    //                             ),
    //                         )));
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
}

impl Visitor for AstOptimizer {
    fn visit_for(&mut self, _forr: &ast::forr::For) {
        
    }
    fn visit_range(&mut self, _start: &ast::Expression, _end: &ast::Expression) {

    }
    fn visit_function_call(&mut self, _call: &ast::expressions::functioncall::FunctionCall) {
        
    }
    fn visit_function_def(&mut self, _def: &ast::expressions::functiondeclaration::FunctionDef) {

        
    }

    fn visit_program(&mut self, program: &ast::Program) {
        program.expression_list.accept(self);
    }

    fn visit_expression_list(&mut self, expr_list: &ast::ExpressionList) {
        for expr in &expr_list.expressions {
            expr.accept(self);
        }
    }

    fn visit_ifelse(&mut self, ifelse: &crate::ast::expressions::ifelse::IfElse) {
        ifelse.condition.accept(self);
        ifelse.then_branch.accept(self);
        for (_, cond, expr) in &ifelse.elif_branches {
            cond.accept(self);
            expr.accept(self);
        }
        if let Some(branch) = &ifelse.else_branch {
            branch.accept(self);
        }
    }

    fn visit_expression(&mut self, expr: &ast::Expression) {
        expr.accept(self);
    }

    fn visit_atom(&mut self, _atom: &ast::atoms::atom::Atom) {
        // Nothing to optimize in atoms
    }

    fn visit_binary_op(&mut self, _binop: &BinaryOp) {
        // Primero, recorre los hijos
        // binop.left.accept(self);
        // binop.right.accept(self);

        // use crate::ast::atoms::atom::Atom::*;
        // use crate::tokens::Literal;

        // // Intenta reducir si ambos lados son literales
        // if let (ast::Expression::Atom(left_atom), ast::Expression::Atom(rightAtom)) = (&*binop.left, &*binop.right) {
        //     match (&**leftAtom, &**rightAtom, &binop.operator) {
        //         // Comparaciones numéricas
        //         (NumberLiteral(Literal::Number(l, _)), NumberLiteral(Literal::Number(r, _)), op) => {
        //             let result = match op {
        //                 BinOp::GreaterEqual(_) => l >= r,
        //                 BinOp::Greater(_) => l > r,
        //                 BinOp::LessEqual(_) => l <= r,
        //                 BinOp::Less(_) => l < r,
        //                 BinOp::EqualEqual(_) => l == r,
        //                 BinOp::NotEqual(_) => l != r,
        //                 _ => return,
        //             };
        //             // Aquí deberías reemplazar el nodo en el AST por un Atom::BooleanLiteral
        //             // (esto requiere un AST mutable o una transformación, aquí solo es ejemplo)
        //             // println!("Reducible: {} {:?} {} => {}", l, op, r, result);
        //         }
        //         // Operadores lógicos entre booleanos
        //         (BooleanLiteral(Literal::Bool(l, _)), BooleanLiteral(Literal::Bool(r, _)), op) => {
        //             let result = match op {
        //                 BinOp::AndAnd(_) => *l && *r,
        //                 BinOp::OrOr(_) => *l || *r,
        //                 BinOp::EqualEqual(_) => l == r,
        //                 BinOp::NotEqual(_) => l != r,
        //                 _ => return,
        //             };
        //             // println!("Reducible: {} {:?} {} => {}", l, op, r, result);
        //         }
        //         _ => {}
        //     }
        }

    fn visit_letin(&mut self, _letin: &ast::expressions::letin::LetIn) {
        // for assign in &letin.assignments {
        //     assign.accept(self);
        // }
        // letin.expression.accept(self);
    }

    fn visit_assignment(&mut self, _assign: &ast::expressions::letin::Assignment) {
        // assign.expression.accept(self);
    }

    fn visit_block(&mut self, _block: &ast::expressions::block::Block) {
        // for expr in &block.expressions.expressions {
        //     expr.accept(self);
        // }
    }

    fn visit_literal(&mut self, _literal: &crate::tokens::Literal) {}

    fn visit_identifier(&mut self, _identifier: &crate::tokens::Identifier) {}

    fn visit_print(&mut self, expr: &ast::Expression) {
        expr.accept(self);
    }

    fn visit_group(&mut self, _group: &ast::atoms::group::Group) {}

    fn visit_unary_op(&mut self, _unary_op: &ast::expressions::unaryoperation::UnaryOp) {}

    fn visit_while(&mut self, _whilee: &ast::whilee::While) {}

    fn visit_functdef(&mut self, _functdef: &ast::expressions::functiondeclaration::FunctionDef) {
        // Nothing to optimize in function definitions
    }

    fn visit_functcall(&mut self, _functcall: &ast::expressions::functioncall::FunctionCall) {
        // Nothing to optimize in function calls
    }

}
// ...existing code...

use std::collections::HashSet;

/// Preprocesador: marca las llamadas a función con '@' antes del parseo.
/// - Busca todas las declaraciones de función y las guarda.
/// - Reemplaza cada llamada a función `foo(` por `@foo(` en el código fuente.
pub fn preprocess_functions(source: &str) -> String {
    let mut function_names = HashSet::new();
    let mut output = String::new();

    // 1. Encuentra todas las declaraciones de función
    for line in source.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("function ") {
            if let Some(name) = rest.split('(').next() {
                let name = name.trim();
                if !name.is_empty() {
                    function_names.insert(name.to_string());
                }
            }
        }
    }

    // 2. Recorre el código y reemplaza llamadas a función
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let ident: String = chars[start..i].iter().collect();

            // Guarda los espacios en una variable temporal
            let mut j = i;
            let mut spaces = String::new();
            while j < chars.len() && chars[j].is_whitespace() {
                spaces.push(chars[j]);
                j += 1;
            }

            let is_function_call = j < chars.len() && chars[j] == '(' && function_names.contains(&ident);
            let prev = output.trim_end();
            let is_definition = prev.ends_with("function");
            if is_function_call && !is_definition {
                output.push('@');
                output.push_str(&ident);
            } else {
                output.push_str(&ident);
            }
            output.push_str(&spaces);
            i = j;
        } else {
            output.push(chars[i]);
            i += 1;
        }
    }

    output
}