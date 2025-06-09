use crate::ast::atoms::atom::Atom;
use crate::ast::expressions::binoperation::BinaryOp;
use crate::ast::expressions::expressions::Expression;
use crate::ast::visitor::visitor::Visitor;
use crate::ast::{ExpressionList, Program};
use crate::tokens::Literal;
use crate::{Visitable, group, identifier, whilee};
use std::collections::HashMap;

pub struct LLVMGenerator {
    pub code: Vec<String>,
    pub temp_count: usize,
    pub last_temp: String,
    pub string_globals: Vec<String>,
    pub env_stack: Vec<HashMap<String, String>>,
}

impl LLVMGenerator {
    pub fn new() -> Self {
        LLVMGenerator {
            code: Vec::new(),
            temp_count: 0,
            last_temp: String::new(),
            string_globals: Vec::new(),
            env_stack: vec![HashMap::new()],
        }
    }
    fn next_temp(&mut self) -> String {
        let t = format!("%t{}", self.temp_count);
        self.temp_count += 1;
        t
    }

    pub fn llvm_header() -> Vec<String> {
        vec![
            "@.fmt_int = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\"".to_string(),
            "@.fmt_str = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\"".to_string(),
            "@.true_str = private unnamed_addr constant [5 x i8] c\"true\\00\"".to_string(),
            "@.false_str = private unnamed_addr constant [6 x i8] c\"false\\00\"".to_string(),
            "declare i32 @printf(i8*, ...)".to_string(),
            "".to_string(),
            "define i32 @main() {".to_string(),
        ]
    }
    pub fn llvm_footer() -> Vec<String> {
        vec!["  ret i32 0".to_string(), "}".to_string()]
    }

    fn lookup_var(&self, name: &str) -> Option<&String> {
        for scope in self.env_stack.iter().rev() {
            if let Some(ptr) = scope.get(name) {
                return Some(ptr);
            }
        }
        None
    }
}

impl Visitor for LLVMGenerator {
    fn visit_program(&mut self, program: &Program) {
        program.expression_list.accept(self);
    }

    fn visit_expression_list(&mut self, expr_list: &ExpressionList) {
        for expr in &expr_list.expressions {
            expr.accept(self);
        }
    }

    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::BinaryOp(binop) => binop.accept(self),
            Expression::Atom(atom) => atom.accept(self),
            Expression::Print(exp, _pos) => self.visit_print(exp),
            Expression::LetIn(letin) => letin.accept(self),
            _ => {}
        }
    }

    fn visit_atom(&mut self, atom: &Atom) {
        match atom {
            Atom::NumberLiteral(lit) => self.visit_literal(lit),
            Atom::BooleanLiteral(lit) => self.visit_literal(lit),
            Atom::StringLiteral(lit) => self.visit_literal(lit),
            Atom::Variable(identifier) => {
                let ptr = self
                    .lookup_var(&identifier.name)
                    .unwrap_or_else(|| panic!("Variable {} not found in scope", identifier.name))
                    .clone();
                let temp = self.next_temp();
                self.code.push(format!(
                    "{temp} = load i32, i32* {ptr}",
                    temp = temp,
                    ptr = ptr
                ));
                self.last_temp = temp;
            }
            Atom::Group(group) => {
                group.accept(self);
            }
        }
    }

    fn visit_binary_op(&mut self, binop: &BinaryOp) {
        binop.left.accept(self);
        let left = self.last_temp.clone();
        binop.right.accept(self);
        let right = self.last_temp.clone();
        let temp = self.next_temp();
        let op = match &binop.operator {
            crate::tokens::BinOp::Plus(_) => "add",
            crate::tokens::BinOp::Minus(_) => "sub",
            crate::tokens::BinOp::Mul(_) => "mul",
            crate::tokens::BinOp::Div(_) => "sdiv",
            _ => "add", // default
        };
        self.code.push(format!("{temp} = {op} i32 {left}, {right}"));
        self.last_temp = temp;
    }

    fn visit_letin(&mut self, letin: &crate::ast::expressions::letin::LetIn) {
        self.env_stack.push(HashMap::new()); // Nuevo scope

        for assign in &letin.bindings {
            let var_name = match &assign.variable {
                Atom::Variable(identifier) => &identifier.name,
                _ => panic!("Expected variable in assignment"),
            };
            let scope_depth = self.env_stack.len();
            let unique_var = format!("{}_{}", var_name, scope_depth);
            self.code.push(format!("%{} = alloca i32", unique_var));
            assign.body.accept(self);
            self.code
                .push(format!("store i32 {}, i32* %{}", self.last_temp, unique_var));
            // Guarda el puntero en el scope actual
            self.env_stack
                .last_mut()
                .unwrap()
                .insert(var_name.clone(), format!("%{}", unique_var));
        }

        letin.body.accept(self);

        self.env_stack.pop(); // Sale del scope
    }

    fn visit_assignment(&mut self, _assign: &crate::ast::expressions::letin::Assignment) {}

    fn visit_block(&mut self, block: &crate::ast::expressions::block::Block) {
        self.env_stack.push(HashMap::new()); // Nuevo scope

        block.expression_list.accept(self);

        self.env_stack.pop(); // Sale del scope
    }

    fn visit_literal(&mut self, literal: &Literal) {
        let temp = self.next_temp();
        match literal {
            Literal::Number(n, _) => {
                self.code.push(format!("{temp} = add i32 0, {n}"));
                self.last_temp = temp;
            }
            Literal::Bool(val, _) => {
                let bool_val = if *val { 1 } else { 0 };
                self.code.push(format!(
                    "{temp} = icmp eq i1 {bool_val}, 1",
                    temp = temp,
                    bool_val = bool_val
                ));
                self.last_temp = temp;
            }
            Literal::Str(s, _) => {
                // Genera un global único para cada string
                let label = format!("@.str_{}", self.temp_count);
                let bytes = s.as_bytes();
                let len = bytes.len() + 1; // +1 para el null terminator
                let mut str_bytes = bytes.to_vec();
                str_bytes.push(0);
                let _str_const = str_bytes
                    .iter()
                    .map(|b| format!("\\{:02X}", b))
                    .collect::<String>();
                self.string_globals.push(format!(
                    "{label} = private unnamed_addr constant [{len} x i8] c\"{s}\\00\"",
                    label = label,
                    len = len,
                    s = s.replace("\\", "\\5C").replace("\"", "\\22")
                ));
                self.last_temp = label;
            }
        }
    }

    fn visit_identifier(&mut self, _identifier: &crate::tokens::Identifier) {}

    fn visit_print(&mut self, expr: &Expression) {
        expr.accept(self);
        // Aquí podrías inspeccionar el tipo de la expresión si tienes esa info.
        // Por simplicidad, asume que es i32 (número), pero puedes mejorar esto.
        self.code.push(format!(
        "call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.fmt_int, i32 0, i32 0), i32 {})",
        self.last_temp
    ));
    }

    fn visit_while(&mut self, _whilee: &whilee::While) {}

    fn visit_ifelse(&mut self, ifelse: &crate::ast::expressions::ifelse::IfElse) {
        let mut labels = Vec::new();

        // Etiquetas para cada rama
        let then_label = self.next_temp();
        labels.push(then_label.clone());

        for _ in &ifelse.elif_branches {
            labels.push(self.next_temp());
        }

        let else_label = if ifelse.else_branch.is_some() {
            self.next_temp()
        } else {
            self.next_temp() // igual se necesita para el salto final
        };
        labels.push(else_label.clone());

        let end_label = self.next_temp();

        // Condición inicial (if)
        ifelse.condition.accept(self);
        let cond_temp = self.last_temp.clone();
        self.code.push(format!(
            "br i1 {cond}, label %{then}, label %{elif_or_else}",
            cond = cond_temp,
            then = &labels[0][1..],
            elif_or_else = &labels[1][1..]
        ));

        // Rama then
        self.code.push(format!("{}:", &labels[0][1..]));
        ifelse.then_branch.accept(self);
        self.code
            .push(format!("br label %{end}", end = &end_label[1..]));

        // Elif branches
        for (i, (_kw, cond, branch)) in ifelse.elif_branches.iter().enumerate() {
            self.code.push(format!("{}:", &labels[i + 1][1..]));
            cond.accept(self);
            let cond_temp = self.last_temp.clone();
            let next_label = if i + 2 < labels.len() {
                &labels[i + 2]
            } else {
                &else_label
            };
            self.code.push(format!(
                "br i1 {cond}, label %{then}, label %{next}",
                cond = cond_temp,
                then = &labels[i + 1][1..],
                next = &next_label[1..]
            ));
            branch.accept(self);
            self.code
                .push(format!("br label %{end}", end = &end_label[1..]));
        }

        // Rama else (si existe)
        if let Some(else_branch) = &ifelse.else_branch {
            self.code.push(format!("{}:", &else_label[1..]));
            else_branch.accept(self);
            self.code
                .push(format!("br label %{end}", end = &end_label[1..]));
        } else {
            // Si no hay else, igual marca el bloque
            self.code.push(format!("{}:", &else_label[1..]));
            self.code
                .push(format!("br label %{end}", end = &end_label[1..]));
        }

        // Fin
        self.code.push(format!("{}:", &end_label[1..]));
    }

    fn visit_group(&mut self, group: &crate::ast::atoms::group::Group) {
        group.expression.accept(self);
    }
}
