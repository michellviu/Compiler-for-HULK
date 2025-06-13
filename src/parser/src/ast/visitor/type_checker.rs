use crate::ast;
// use crate::ast::expressions::binoperation::BinaryOp;
use crate::ast::visitor::{Visitable, Visitor};
use crate::tokens;
// use crate::tokens::BinOp;
use crate::visitor::Type;

use std::collections::HashMap;

pub struct TypeChecker {
    pub errors: Vec<String>,
    scopes: Vec<HashMap<String, Type>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            errors: Vec::new(),
            scopes: vec![HashMap::new()],
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_var(&mut self, name: &str, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), ty);
        }
    }

    fn lookup_var(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }
}

impl Visitor for TypeChecker {
    fn visit_access_type_prop(&mut self, access: &ast::expressions::accesstypesprop::AccessTypeProp) {
        
    }
    fn visit_declaration_function(&mut self, decl: &ast::expressions::declarationtypes::Declarationtypes) {
        
    }
    fn visit_instanting_types(&mut self, inst: &ast::expressions::instantiatingtypes::InstantingTypes) {
        
    }
    fn visit_type_declaration(&mut self, decl: &ast::expressions::declarationtypes::Declarationtypes) {
        
    }
    fn visit_atom(&mut self, _atom: &ast::atoms::atom::Atom) {
        
    }
    fn visit_block(&mut self, _block: &ast::expressions::block::Block) {
        
    }
    fn visit_expression(&mut self, _expr: &ast::Expression) {
        
    }
    fn visit_expression_list(&mut self, expr_list: &ast::ExpressionList) {
        for expr in &expr_list.expressions {
            expr.accept(self);
        }
    }
    fn visit_for(&mut self, _forr: &ast::forr::For) {
        
    }
    fn visit_group(&mut self, _group: &ast::atoms::group::Group) {
        
    }
    fn visit_identifier(&mut self, _identifier: &tokens::Identifier) {
        
    }
    fn visit_literal(&mut self, _literal: &tokens::Literal) {
        
    }
    fn visit_print(&mut self, expr: &ast::Expression) {
        expr.accept(self);
    }
    fn visit_program(&mut self, program: &ast::Program) {
        program.expression_list.accept(self);
    }
    fn visit_range(&mut self, _start: &ast::Expression, _end: &ast::Expression) {
        
    }
    fn visit_unary_op(&mut self, _unary_op: &ast::expressions::unaryoperation::UnaryOp) {
        
    }

    fn visit_letin(&mut self, letin: &crate::ast::expressions::letin::LetIn) {
        self.enter_scope();
        for assign in &letin.bindings {
            assign.accept(self);
        }
        letin.body.accept(self);
        self.exit_scope();
    }

    fn visit_assignment(&mut self, assign: &crate::ast::expressions::letin::Assignment) {
        let var_name = if let crate::ast::atoms::atom::Atom::Variable(id) = &assign.variable {
            id.name.clone()
        } else {
            "<anon>".to_string()
        };
        assign.body.accept(self);
        let ty = self.infer_expr_type(&assign.body);
        self.declare_var(&var_name, ty);
    }

    fn visit_binary_op(&mut self, binop: &crate::ast::expressions::binoperation::BinaryOp) {
        binop.left.accept(self);
        binop.right.accept(self);
        use crate::tokens::BinOp;
        let left_ty = self.infer_expr_type(&binop.left);
        let right_ty = self.infer_expr_type(&binop.right);

        match &binop.operator {
            BinOp::Plus(_) | BinOp::Minus(_) | BinOp::Mul(_) | BinOp::Div(_) | BinOp::Mod(_) => {
                if left_ty != Type::Number || right_ty != Type::Number {
                    self.errors.push("Operación aritmética requiere números".to_string());
                }
            }
            BinOp::EqualEqual(_) | BinOp::NotEqual(_) | BinOp::Greater(_)
            | BinOp::Less(_) | BinOp::GreaterEqual(_) | BinOp::LessEqual(_) => {
                if left_ty != right_ty {
                    self.errors.push("Comparación entre tipos incompatibles".to_string());
                }
            }
            BinOp::AndAnd(_) | BinOp::OrOr(_) => {
                if left_ty != Type::Boolean || right_ty != Type::Boolean {
                    self.errors.push("Operador lógico requiere booleanos".to_string());
                }
            }
            BinOp::ConcatString(_) => {
                if !(left_ty == Type::String || left_ty == Type::Number) ||
                   !(right_ty == Type::String || right_ty == Type::Number) {
                    self.errors.push("Concatenación requiere string o número".to_string());
                }
            }
            BinOp::Assign(_) => { /* handled in assignment */ }
            _ => {}
        }
    }

    fn visit_ifelse(&mut self, ifelse: &crate::ast::expressions::ifelse::IfElse) {
        ifelse.condition.accept(self);
        let cond_ty = self.infer_expr_type(&ifelse.condition);
        if cond_ty != Type::Boolean {
            self.errors.push("Condición de if debe ser booleana".to_string());
        }
        ifelse.then_branch.accept(self);
        for (_, cond, branch) in &ifelse.elif_branches {
            cond.accept(self);
            let t = self.infer_expr_type(cond);
            if t != Type::Boolean {
                self.errors.push("Condición de elif debe ser booleana".to_string());
            }
            branch.accept(self);
        }
        if let Some(branch) = &ifelse.else_branch {
            branch.accept(self);
        }
    }

    fn visit_while(&mut self, whilee: &crate::ast::expressions::whilee::While) {
        whilee.cond.accept(self);
        let cond_ty = self.infer_expr_type(&whilee.cond);
        if cond_ty != Type::Boolean {
            self.errors.push("Condición de while debe ser booleana".to_string());
        }
        whilee.body.accept(self);
    }
    fn visit_function_call(&mut self, _call: &ast::expressions::functioncall::FunctionCall) {
        
    }
    fn visit_function_def(&mut self, _def: &ast::expressions::functiondeclaration::FunctionDef) {
        
    }

    // ...
}

impl TypeChecker {
    fn infer_expr_type(&mut self, expr: &crate::ast::Expression) -> Type {
        use crate::ast::Expression;
        use crate::ast::atoms::atom::Atom;
        match expr {
            Expression::Atom(atom) => match &**atom {
                Atom::NumberLiteral(_) => Type::Number,
                Atom::BooleanLiteral(_) => Type::Boolean,
                Atom::StringLiteral(_) => Type::String,
                Atom::Variable(id) => {
                    match self.lookup_var(&id.name) {
                        Some(ty) => ty,
                        None => {
                            self.errors.push(format!("Variable '{}' no está declarada", id.name));
                            Type::Unknown
                        }
                    }
                }
                _ => Type::Unknown,
            },
            Expression::BinaryOp(binop) => {
                use crate::tokens::BinOp;
                let _left = self.infer_expr_type(&binop.left);
                let _right = self.infer_expr_type(&binop.right);
                match &binop.operator {
                    BinOp::Plus(_) | BinOp::Minus(_) | BinOp::Mul(_) | BinOp::Div(_) | BinOp::Mod(_) => Type::Number,
                    BinOp::EqualEqual(_) | BinOp::NotEqual(_) | BinOp::Greater(_)
                    | BinOp::Less(_) | BinOp::GreaterEqual(_) | BinOp::LessEqual(_)
                    | BinOp::AndAnd(_) | BinOp::OrOr(_) => Type::Boolean,
                    BinOp::ConcatString(_) => Type::String,
                    _ => Type::Unknown,
                }
            }
            _ => Type::Unknown,
        }
    }
}