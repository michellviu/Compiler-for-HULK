use std::f32::consts::E;

use crate::ast;
use crate::ast::Expression;
use crate::ast::atoms::atom::Atom;
use crate::forr;
use crate::group;
use crate::tokens;
use crate::visitor::Visitable;
use crate::visitor::Visitor;
use crate::whilee;

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
    fn visit_access_type_prop(
        &mut self,
        access: &ast::expressions::accesstypesprop::AccessTypeProp,
    ) {
        println!("{}TypeMethodAccess: {}", self.pad(), access.properties.name);
        self.indent += 1;
        println!("{}Instance:", self.pad());
        self.indent += 1;
        access.referenced_type.accept(self);
        self.indent -= 1;
        if !access.params.is_empty() {
            println!("{}Args:", self.pad());
            self.indent += 1;
            for arg in &access.params {
                arg.accept(self);
            }
            self.indent -= 1;
        }
        self.indent -= 1;
    }

    fn visit_instanting_types(
        &mut self,
        inst: &ast::expressions::instantiatingtypes::InstantingTypes,
    ) {
        println!(
            "{}TypeInstantiation: {}",
            self.pad(),
            inst.referenced_type.name
        );
        self.indent += 1;
        if !inst.params.is_empty() {
            println!("{}Args:", self.pad());
            self.indent += 1;
            for arg in &inst.params {
                arg.accept(self);
            }
            self.indent -= 1;
        }
        self.indent -= 1;
    }
    fn visit_type_declaration(
        &mut self,
        decl: &ast::expressions::declarationtypes::Declarationtypes,
    ) {
        println!(">>> Entrando a visit_type_declaration");
        println!("{}TypeDeclaration: {}", self.pad(), decl.name_types.name);
        self.indent += 1;

        // Imprime parámetros de tipo
        if !decl.build.is_empty() {
            println!("{}TypeParams:", self.pad());
            self.indent += 1;
            for param in &decl.build {
                println!("{}{}", self.pad(), param.name);
            }
            self.indent -= 1;
        }

        // Imprime propiedades
        if !decl.properties.is_empty() {
            println!("{}Properties:", self.pad());
            self.indent += 1;
            for prop in &decl.properties {
                prop.accept(self);
            }
            self.indent -= 1;
        }

        // Imprime métodos
        if !decl.functions.is_empty() {
            println!("{}Methods:", self.pad());
            self.indent += 1;
            for func in &decl.functions {
                func.accept(self);
            }
            self.indent -= 1;
        }

        self.indent -= 1;
    }

    fn visit_program(&mut self, program: &ast::Program) {
        println!("{}Program", self.pad());
        self.indent += 1;
        // Imprime las funciones declaradas
        // for func in &program.functions {
        //     func.accept(self);
        // }
        // Imprime la lista de expresiones globales
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
            Expression::BinaryOp(binop) => binop.accept(self),
            Expression::Atom(atom) => atom.accept(self),
            Expression::Print(expr, _pos) => self.visit_print(expr),
            Expression::While(whilee) => whilee.accept(self),
            Expression::IfElse(ifelse) => ifelse.accept(self),
            Expression::LetIn(letin) => letin.accept(self),
            Expression::Block(block) => block.accept(self),
            Expression::UnaryOp(unary_op) => unary_op.accept(self),
            Expression::For(forr) => forr.accept(self),
            Expression::Range(start, end) => self.visit_range(start, end),
            Expression::FunctionCall(call) => call.accept(self),
            Expression::FunctionDef(def) => def.accept(self),
            Expression::TypeDeclaration(decl) => decl.accept(self),
            Expression::TypeInstantiation(inst) => inst.accept(self),
            Expression::TypeMethodAccess(access) => access.accept(self),
            Expression::TypePropertyAccess(access) => access.accept(self),
        }
    }

    fn visit_range(&mut self, start: &ast::Expression, end: &ast::Expression) {
        println!("{}Range", self.pad());
        self.indent += 1;
        println!("{}Start:", self.pad());
        self.indent += 1;
        start.accept(self);
        self.indent -= 1;
        println!("{}End:", self.pad());
        self.indent += 1;
        end.accept(self);
        self.indent -= 2;
    }
    fn visit_function_call(&mut self, call: &ast::expressions::functioncall::FunctionCall) {
        println!("{}FunctionCall: {}", self.pad(), call.funct_name);
        self.indent += 1;
        for arg in &call.arguments {
            arg.accept(self);
        }
        self.indent -= 1;
    }

    fn visit_function_def(&mut self, def: &ast::expressions::functiondeclaration::FunctionDef) {
        println!("{}FunctionDef: {}", self.pad(), def.name);
        self.indent += 1;
        println!("{}Params:", self.pad());
        self.indent += 1;
        for param in &def.params {
            println!("{}{}", self.pad(), param.name);
        }
        self.indent -= 1;
        println!("{}Body:", self.pad());
        self.indent += 1;
        def.body.accept(self);
        self.indent -= 2;
    }
    fn visit_atom(&mut self, atom: &ast::atoms::atom::Atom) {
        use crate::ast::atoms::atom::Atom::*;
        match atom {
            NumberLiteral(lit) => lit.accept(self),
            BooleanLiteral(lit) => lit.accept(self),
            StringLiteral(lit) => lit.accept(self),
            Variable(id) => {
                println!("{}Variable: {}", self.pad(), id.name);
            }
            Group(expr) => expr.accept(self),
        }
    }
    fn visit_for(&mut self, forr: &forr::For) {
        println!("{}For", self.pad());
        self.indent += 1;
        println!("{}Var:", self.pad());
        self.indent += 1;
        forr.var.accept(self);
        self.indent -= 1;
        println!("{}Iterable:", self.pad());
        self.indent += 1;
        forr.iterable.accept(self);
        self.indent -= 1;
        println!("{}Body:", self.pad());
        self.indent += 1;
        forr.body.accept(self);
        self.indent -= 2;
    }

    fn visit_binary_op(&mut self, binop: &ast::expressions::binoperation::BinaryOp) {
        use crate::tokens::BinOp;
        match &binop.operator {
            BinOp::Assign(_) => {
                println!("{}DestructiveAssign:", self.pad());
                self.indent += 1;
                binop.left.accept(self);
                binop.right.accept(self);
                self.indent -= 1;
            }
            _ => {
                println!("{}BinaryOp: {}", self.pad(), binop.operator);
                self.indent += 1;
                binop.left.accept(self);
                binop.right.accept(self);
                self.indent -= 1;
            }
        }
    }
    fn visit_letin(&mut self, letin: &ast::expressions::letin::LetIn) {
        println!("{}LetIn", self.pad());
        self.indent += 1;
        for assign in &letin.bindings {
            assign.accept(self);
        }
        letin.body.accept(self);
        self.indent -= 1;
    }
    fn visit_ifelse(&mut self, ifelse: &ast::expressions::ifelse::IfElse) {
        println!("{}If", self.pad());
        self.indent += 1;
        println!("{}Condition:", self.pad());
        self.indent += 1;
        ifelse.condition.accept(self);
        self.indent -= 1;

        println!("{}Then:", self.pad());
        self.indent += 1;
        ifelse.then_branch.accept(self);
        self.indent -= 1;

        for (_elif_kw, cond, branch) in &ifelse.elif_branches {
            println!("{}Elif:", self.pad());
            self.indent += 1;
            cond.accept(self);
            branch.accept(self);
            self.indent -= 1;
        }

        if let Some(else_expr) = &ifelse.else_branch {
            println!("{}Else:", self.pad());
            self.indent += 1;
            else_expr.accept(self);
            self.indent -= 1;
        }

        self.indent -= 1;
    }
    fn visit_assignment(&mut self, assign: &ast::expressions::letin::Assignment) {
        let var_name = match &assign.variable {
            Atom::Variable(identifier) => &identifier.name,
            _ => panic!("Expected variable in assignment"),
        };
        println!("{}Assignment: {} {}", self.pad(), var_name, assign.op);
        self.indent += 1;
        assign.body.accept(self);
        self.indent -= 1;
    }
    fn visit_block(&mut self, block: &ast::expressions::block::Block) {
        println!("{}Block", self.pad());
        self.indent += 1;
        block.expression_list.accept(self);
        self.indent -= 1;
    }
    fn visit_literal(&mut self, literal: &tokens::Literal) {
        let type_str = match literal {
            tokens::Literal::Number(_, _) => "Number",
            tokens::Literal::Str(_, _) => "String",
            tokens::Literal::Bool(_, _) => "Bool",
        };
        println!("{}{}Literal: {}", self.pad(), type_str, literal);
    }
    fn visit_identifier(&mut self, identifier: &tokens::Identifier) {
        println!("{}Identifier: {}", self.pad(), identifier);
    }
    fn visit_print(&mut self, expr: &ast::Expression) {
        println!("{}Print", self.pad());
        self.indent += 1;
        expr.accept(self);
        self.indent -= 1;
    }
    fn visit_while(&mut self, whilee: &whilee::While) {
        println!("{}While", self.pad());
        self.indent += 1;
        println!("{}Condition:", self.pad());
        self.indent += 1;
        whilee.cond.accept(self);
        self.indent -= 1;
        println!("{}Body:", self.pad());
        self.indent += 1;
        whilee.body.accept(self);
        self.indent -= 2;
    }

    fn visit_group(&mut self, group: &group::Group) {
        println!("{}Group", self.pad());
        group.expression.accept(self);
    }

    fn visit_unary_op(&mut self, unary_op: &ast::expressions::unaryoperation::UnaryOp) {
        println!("{}UnaryOp: {}", self.pad(), unary_op.op);
        self.indent += 1;
        unary_op.expr.accept(self);
        self.indent -= 1;
    }
}
