use crate::Visitable;
use crate::ast::atoms::atom::Atom;
use crate::ast::expressions::binoperation::BinaryOp;
use crate::ast::expressions::expressions::Expression;
use crate::ast::visitor::visitor::Visitor;
use crate::ast::{ExpressionList, Program};
use crate::tokens::Literal;

pub struct LLVMGenerator {
    pub code: Vec<String>,
    pub temp_count: usize,
    pub last_temp: String,
    pub string_globals: Vec<String>, // <-- Añade esto
}

impl LLVMGenerator {
    pub fn new() -> Self {
        LLVMGenerator {
            code: Vec::new(),
            temp_count: 0,
            last_temp: String::new(),
            string_globals: Vec::new(),
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
            Expression::BinaryOp(binop) => self.visit_binary_op(binop),
            Expression::Atom(atom) => atom.accept(self),
            _ => {}
        }
    }
    fn visit_atom(&mut self, atom: &Atom) {
        match atom {
            Atom::NumberLiteral(lit) => self.visit_literal(lit),
            Atom::BooleanLiteral(lit) => self.visit_literal(lit),

            _ => {}
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
    fn visit_letin(&mut self, _letin: &crate::ast::expressions::letin::LetIn) {}
    fn visit_assignment(&mut self, _assign: &crate::ast::expressions::letin::Assignment) {}
    fn visit_block(&mut self, _block: &crate::ast::atoms::block::Block) {}
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
                let str_const = str_bytes
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
        match expr {
            Expression::Atom(atom) => match &**atom {
                Atom::NumberLiteral(_) => {
                    expr.accept(self);
                    let value_temp = self.last_temp.clone();
                    self.code.push(format!(
                    "call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.fmt_int, i32 0, i32 0), i32 {value})",
                    value = value_temp
                ));
                }
                Atom::BooleanLiteral(lit) => {
                    // Evalúa el booleano
                    expr.accept(self);
                    let value_temp = self.last_temp.clone();
                    let true_label = self.next_temp();
                    let false_label = self.next_temp();
                    let end_label = self.next_temp();
                    let result_temp = self.next_temp();
                    // Selecciona la cadena "true" o "false"
                    self.code.push(format!(
                        "br i1 {cond}, label %{true_label}, label %{false_label}",
                        cond = value_temp,
                        true_label = &true_label[1..], // quita el %
                        false_label = &false_label[1..]
                    ));
                    self.code.push(format!("{}:", &true_label[1..]));
                    self.code.push(format!(
                    "  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_str, i32 0, i32 0))"
                ));
                    self.code.push(format!(
                        "  br label %{end_label}",
                        end_label = &end_label[1..]
                    ));
                    self.code.push(format!("{}:", &false_label[1..]));
                    self.code.push(format!(
                    "  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_str, i32 0, i32 0))"
                ));
                    self.code.push(format!(
                        "  br label %{end_label}",
                        end_label = &end_label[1..]
                    ));
                    self.code.push(format!("{}:", &end_label[1..]));
                }
                Atom::StringLiteral(lit) => {
                    // Suponiendo que tienes la cadena en el IR como un global
                    let str_label = format!("@.str_{}", self.temp_count); // Debes generar y declarar el global
                    self.code.push(format!(
                    "call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([N x i8], [N x i8]* {label}, i32 0, i32 0))",
                    label = str_label
                ));
                }
                _ => {}
            },
            _ => {}
        }
    }
    fn visit_while(&mut self, _cond: &Expression, _body: &Expression) {}
    fn visit_ifelse(&mut self, _ifelse: &crate::ast::expressions::ifelse::IfElse) {}
}
