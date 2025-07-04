use super::symbol_table::{SymbolInfo, SymbolTable};
use super::types::Type;
use crate::ast::atoms::atom::Atom;
use crate::ast::expressions::binoperation::BinaryOp;
use crate::ast::expressions::expressions::Expression;
use crate::ast::visitor::visitor::Visitor;
use crate::ast::{ExpressionList, Program};
use crate::tokens::Literal;
use crate::{Visitable, whilee};
use std::collections::HashMap;

pub struct LLVMGenerator {
    pub code: Vec<String>,
    pub functions: Vec<String>,
    pub temp_count: usize,
    pub last_temp: String,
    pub string_globals: Vec<String>,
    pub env_stack: Vec<HashMap<String, String>>,
    pub string_sizes: HashMap<String, usize>,
    pub string_label_count: usize,
    pub symbol_table: SymbolTable,
}

impl LLVMGenerator {
    pub fn new(symbol_table: SymbolTable) -> Self {
        LLVMGenerator {
            code: Vec::new(),
            functions: Vec::new(),
            temp_count: 0,
            last_temp: String::new(),
            string_globals: Vec::new(),
            env_stack: vec![HashMap::new()],
            string_sizes: HashMap::new(),
            string_label_count: 0,
            symbol_table,
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

    fn visit_range(&mut self, _start: &crate::ast::Expression, _end: &crate::ast::Expression) {}

    fn visit_function_call(&mut self, call: &crate::ast::expressions::functioncall::FunctionCall) {
        let (ret_type, param_types) = match self.symbol_table.lookup(&call.funct_name.name) {
            Some(SymbolInfo::Function {
                return_type,
                param_types,
            }) => (return_type.clone(), param_types.clone()),
            _ => panic!("'{}' no es una función", call.funct_name.name),
        };

        // Evalúa los argumentos y guarda los temporales
        let mut arg_temps = Vec::new();
        for arg in &call.arguments {
            arg.accept(self);
            arg_temps.push(self.last_temp.clone());
        }

        // Prepara la lista de argumentos para el call con el tipo correcto
        let args_llvm = arg_temps
            .iter()
            .zip(param_types.iter())
            .map(|(t, ty)| match ty {
                Type::Number => format!("i32 {}", t),
                Type::Boolean => format!("i1 {}", t),
                Type::String => format!("i8* {}", t),
                Type::Custom(_) => format!("i32 {}", t), // Ajusta según tu representación
                _ => panic!("Tipo de argumento no soportado"),
            })
            .collect::<Vec<_>>()
            .join(", ");

        // Determina el tipo de retorno LLVM
        let ret_llvm = match ret_type {
            Type::Number => "i32",
            Type::Boolean => "i1",
            Type::String => "i8*",
            Type::Custom(_) => "i32", // Ajusta según tu representación
            _ => panic!("Tipo de retorno no soportado"),
        };

        // Llama a la función y guarda el resultado en un temporal
        let temp = self.next_temp();
        self.code.push(format!(
            "{temp} = call {ret_llvm} @{name}({args})",
            temp = temp,
            ret_llvm = ret_llvm,
            name = call.funct_name.name,
            args = args_llvm
        ));
        self.last_temp = temp.clone();
    }

    fn visit_function_def(
        &mut self,
        def: &crate::ast::expressions::functiondeclaration::FunctionDef,
    ) {
        let mut fn_code = Vec::new();
        let fn_name = &def.name.name;

        let (ret_type, param_types) = match self.symbol_table.lookup(fn_name) {
            Some(SymbolInfo::Function {
                return_type,
                param_types,
            }) => (return_type.clone(), param_types.clone()),
            _ => panic!(
                "Función '{}' no encontrada en la tabla de símbolos",
                fn_name
            ),
        };

        let params_llvm = param_types
            .iter()
            .enumerate()
            .map(|(i, ty)| match ty {
                Type::Number => format!("i32 %p{i}"),
                Type::Boolean => format!("i1 %p{i}"),
                Type::String => format!("i8* %p{i}"),
                Type::Custom(_) => format!("i32 %p{i}"), // Ajusta si tienes structs
                _ => panic!("Tipo de parámetro no soportado"),
            })
            .collect::<Vec<_>>()
            .join(", ");

        let ret_llvm = match ret_type {
            Type::Number => "i32".to_string(),
            Type::Boolean => "i1".to_string(),
            Type::String => "i8*".to_string(),
            Type::Custom(name) => format!("%{}*", name), // Ajusta si tienes structs
            _ => panic!("Tipo de retorno no soportado"),
        };

        fn_code.push(format!(
            "define {} @{}({}) {{",
            ret_llvm, fn_name, params_llvm
        ));

        self.env_stack.push(HashMap::new());
        for (i, param) in def.params.iter().enumerate() {
            let unique_var = format!("{}_{}", param.name.name, self.env_stack.len());
            let llvm_type = match &param.signature {
                Type::Number => "i32",
                Type::Boolean => "i1",
                Type::String => "i8*",
                Type::Custom(_) => "i32", // Ajusta si tienes structs
                _ => panic!("Tipo de parámetro no soportado"),
            };
            fn_code.push(format!("%{unique_var} = alloca {llvm_type}"));
            fn_code.push(format!(
                "store {llvm_type} %p{i}, {llvm_type}* %{unique_var}"
            ));
            self.env_stack
                .last_mut()
                .unwrap()
                .insert(param.name.name.clone(), format!("%{unique_var}"));
        }

        // Guarda el código generado temporalmente
        let old_code = std::mem::replace(&mut self.code, Vec::new());
        def.body.accept(self);
        fn_code.extend(self.code.drain(..));
        fn_code.push(format!("ret {} {}", ret_llvm, self.last_temp));
        self.env_stack.pop();
        fn_code.push("}".to_string());
        self.code = old_code;

        self.functions.extend(fn_code);
    }

    fn visit_expression_list(&mut self, expr_list: &ExpressionList) {
        for expr in &expr_list.expressions {
            expr.accept(self);
        }
    }

    fn visit_expression(&mut self, expr: &Expression) {
        expr.accept(self);
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
        use crate::tokens::BinOp;
        match &binop.operator {
            BinOp::Assign(_) => {
                // Lado izquierdo debe ser una variable
                if let Expression::Atom(atom) = &*binop.left {
                    if let Atom::Variable(identifier) = &**atom {
                        let ptr = self
                            .lookup_var(&identifier.name)
                            .unwrap_or_else(|| {
                                panic!("Variable {} not found in scope", identifier.name)
                            })
                            .clone();
                        binop.right.accept(self);
                        let value = self.last_temp.clone();
                        self.code.push(format!("store i32 {}, i32* {}", value, ptr));
                        self.last_temp = value; // := devuelve el valor asignado
                        return;
                    }
                }
                panic!("Left side of := must be a variable");
            }
            // Operadores booleanos y de comparación
            BinOp::EqualEqual(_)
            | BinOp::NotEqual(_)
            | BinOp::Less(_)
            | BinOp::LessEqual(_)
            | BinOp::Greater(_)
            | BinOp::GreaterEqual(_) => {
                binop.left.accept(self);
                let left = self.last_temp.clone();
                binop.right.accept(self);
                let right = self.last_temp.clone();
                let temp = self.next_temp();
                let op = match &binop.operator {
                    BinOp::EqualEqual(_) => "eq",
                    BinOp::NotEqual(_) => "ne",
                    BinOp::Less(_) => "slt",
                    BinOp::LessEqual(_) => "sle",
                    BinOp::Greater(_) => "sgt",
                    BinOp::GreaterEqual(_) => "sge",
                    _ => unreachable!(),
                };
                self.code.push(format!(
                    "{temp} = icmp {op} i32 {left}, {right}",
                    temp = temp,
                    op = op,
                    left = left,
                    right = right
                ));
                self.last_temp = temp;
            }
            // Operadores lógicos (AND, OR)
            BinOp::AndAnd(_) | BinOp::OrOr(_) => {
                binop.left.accept(self);
                let left = self.last_temp.clone();
                binop.right.accept(self);
                let right = self.last_temp.clone();
                let temp = self.next_temp();
                let op = match &binop.operator {
                    BinOp::AndAnd(_) => "and",
                    BinOp::OrOr(_) => "or",
                    _ => unreachable!(),
                };
                self.code.push(format!(
                    "{temp} = {op} i1 {left}, {right}",
                    temp = temp,
                    op = op,
                    left = left,
                    right = right
                ));
                self.last_temp = temp;
            }
            // Operadores aritméticos
            _ => {
                binop.left.accept(self);
                let left = self.last_temp.clone();
                binop.right.accept(self);
                let right = self.last_temp.clone();
                let temp = self.next_temp();
                let op = match &binop.operator {
                    BinOp::Plus(_) => "add",
                    BinOp::Minus(_) => "sub",
                    BinOp::Mul(_) => "mul",
                    BinOp::Div(_) => "sdiv",
                    BinOp::Mod(_) => "srem",
                    _ => "add",
                };
                self.code.push(format!("{temp} = {op} i32 {left}, {right}"));
                self.last_temp = temp;
            }
        }
    }

    fn visit_for(&mut self, forr: &crate::forr::For) {
        use crate::ast::atoms::atom::Atom;
        use crate::ast::expressions::expressions::Expression;

        // Extrae el nombre de la variable de control
        let var_name = if let Expression::Atom(atom) = &*forr.var {
            if let Atom::Variable(identifier) = &**atom {
                &identifier.name
            } else {
                panic!("For variable must be an identifier");
            }
        } else {
            panic!("For variable must be an identifier expression");
        };

        // Crea variable local para el for (scope actual)
        let scope_depth = self.env_stack.len();
        let unique_var = format!("{}_{}", var_name, scope_depth);
        self.code.push(format!("%{} = alloca i32", unique_var));

        // Inicializa variable (asume que el iterable es un rango: range(start, end))
        // Evaluamos el start
        if let Expression::Range(start, end) = &*forr.iterable {
            start.accept(self);
            let start_temp = self.last_temp.clone();
            self.code
                .push(format!("store i32 {}, i32* %{}", start_temp, unique_var));
            // Evaluamos el end
            end.accept(self);
            let end_temp = self.last_temp.clone();

            // Etiquetas
            let loop_cond = self.next_temp();
            let loop_body = self.next_temp();
            let loop_exit = self.next_temp();

            // Guardamos el puntero en el scope
            self.env_stack
                .last_mut()
                .unwrap()
                .insert(var_name.to_string(), format!("%{}", unique_var));

            // Salto a condición
            self.code
                .push(format!("br label %{cond}", cond = &loop_cond[1..]));

            // Condición
            self.code.push(format!("{}:", &loop_cond[1..]));
            let x_val = self.next_temp();
            self.code.push(format!(
                "{x_val} = load i32, i32* %{var}",
                x_val = x_val,
                var = unique_var
            ));
            let cmp = self.next_temp();
            self.code.push(format!(
                "{cmp} = icmp slt i32 {x_val}, {end}",
                cmp = cmp,
                x_val = x_val,
                end = end_temp
            ));
            self.code.push(format!(
                "br i1 {cmp}, label %{body}, label %{exit}",
                cmp = cmp,
                body = &loop_body[1..],
                exit = &loop_exit[1..]
            ));

            // Cuerpo
            self.code.push(format!("{}:", &loop_body[1..]));
            forr.body.accept(self);

            // Incremento
            let x_val2 = self.next_temp();
            self.code.push(format!(
                "{x_val2} = load i32, i32* %{var}",
                x_val2 = x_val2,
                var = unique_var
            ));
            let inc = self.next_temp();
            self.code.push(format!(
                "{inc} = add i32 {x_val2}, 1",
                inc = inc,
                x_val2 = x_val2
            ));
            self.code.push(format!(
                "store i32 {inc}, i32* %{var}",
                inc = inc,
                var = unique_var
            ));
            self.code
                .push(format!("br label %{cond}", cond = &loop_cond[1..]));

            // Exit
            self.code.push(format!("{}:", &loop_exit[1..]));
        } else {
            panic!("For iterable must be a range expression");
        }
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
            self.code.push(format!(
                "store i32 {}, i32* %{}",
                self.last_temp, unique_var
            ));
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
                let label = format!("@.str_{}", self.string_label_count);
                let bytes = s.as_bytes();
                let len = bytes.len() + 1;
                self.string_globals.push(format!(
                    "{label} = private unnamed_addr constant [{len} x i8] c\"{s}\\00\"",
                    label = label,
                    len = len,
                    s = s.replace("\\", "\\5C").replace("\"", "\\22")
                ));
                self.string_sizes.insert(label.clone(), len);
                self.last_temp = label;
                self.string_label_count += 1;
            }
        }
    }

    fn visit_identifier(&mut self, _identifier: &crate::tokens::Identifier) {}

    fn visit_print(&mut self, expr: &Expression) {
        use crate::ast::atoms::atom::Atom;
        expr.accept(self);

        if let Expression::Atom(atom) = expr {
            if let Atom::StringLiteral(_) = &**atom {
                let label = self.last_temp.clone();
                let len = *self.string_sizes.get(&label).unwrap_or(&0);
                let fmt_ptr = self.next_temp();
                let str_ptr = self.next_temp();
                self.code.push(format!(
                    "{fmt_ptr} = getelementptr [4 x i8], [4 x i8]* @.fmt_str, i32 0, i32 0",
                    fmt_ptr = fmt_ptr
                ));
                self.code.push(format!(
                    "{str_ptr} = getelementptr [{len} x i8], [{len} x i8]* {label}, i32 0, i32 0",
                    str_ptr = str_ptr,
                    len = len,
                    label = label
                ));
                self.code.push(format!(
                    "call i32 (i8*, ...) @printf(i8* {fmt_ptr}, i8* {str_ptr})",
                    fmt_ptr = fmt_ptr,
                    str_ptr = str_ptr
                ));
                return;
            }
            if let Atom::BooleanLiteral(_) = &**atom {
                // Imprime como "true"/"false"
                let bool_temp = self.last_temp.clone();
                let true_ptr = self.next_temp();
                let false_ptr = self.next_temp();
                let result_ptr = self.next_temp();
                self.code.push(format!(
                "{true_ptr} = getelementptr inbounds [5 x i8], [5 x i8]* @.true_str, i32 0, i32 0"
            ));
                self.code.push(format!(
                "{false_ptr} = getelementptr inbounds [6 x i8], [6 x i8]* @.false_str, i32 0, i32 0"
            ));
                self.code.push(format!(
                    "{result_ptr} = select i1 {cond}, i8* {true_ptr}, i8* {false_ptr}",
                    result_ptr = result_ptr,
                    cond = bool_temp,
                    true_ptr = true_ptr,
                    false_ptr = false_ptr
                ));
                let fmt_ptr = self.next_temp();
                self.code.push(format!(
                    "{fmt_ptr} = getelementptr [4 x i8], [4 x i8]* @.fmt_str, i32 0, i32 0",
                    fmt_ptr = fmt_ptr
                ));
                self.code.push(format!(
                    "call i32 (i8*, ...) @printf(i8* {fmt_ptr}, i8* {result_ptr})",
                    fmt_ptr = fmt_ptr,
                    result_ptr = result_ptr
                ));
                return;
            }
        }
        self.code.push(format!(
            "call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.fmt_int, i32 0, i32 0), i32 {})",
            self.last_temp
        ));
    }

    fn visit_while(&mut self, whilee: &whilee::While) {
        let cond_label = self.next_temp();
        let body_label = self.next_temp();
        let end_label = self.next_temp();

        // Salto a la condición
        self.code
            .push(format!("br label %{cond}", cond = &cond_label[1..]));

        // Etiqueta de condición
        self.code.push(format!("{}:", &cond_label[1..]));
        whilee.cond.accept(self);
        let cond_temp = self.last_temp.clone();
        self.code.push(format!(
            "br i1 {cond}, label %{body}, label %{end}",
            cond = cond_temp,
            body = &body_label[1..],
            end = &end_label[1..]
        ));

        // Etiqueta de cuerpo
        self.code.push(format!("{}:", &body_label[1..]));
        whilee.body.accept(self);
        // Al terminar el cuerpo, vuelve a la condición
        self.code
            .push(format!("br label %{cond}", cond = &cond_label[1..]));

        // Etiqueta de fin
        self.code.push(format!("{}:", &end_label[1..]));
    }

    fn visit_ifelse(&mut self, ifelse: &crate::ast::expressions::ifelse::IfElse) {
        // Genera etiquetas únicas para cada bloque
        let then_label = format!("then{}", self.temp_count);
        let else_label = format!("else{}", self.temp_count);
        let merge_label = format!("merge{}", self.temp_count);
        self.temp_count += 1;

        // Evalúa la condición principal
        ifelse.condition.accept(self);
        let cond_temp = self.last_temp.clone();

        // Salto condicional
        self.code.push(format!(
            "br i1 {}, label %{}, label %{}",
            cond_temp, then_label, else_label
        ));

        // THEN branch
        self.code.push(format!("{}:", then_label));
        ifelse.then_branch.accept(self);
        let then_result = self.last_temp.clone();
        self.code.push(format!("br label %{}", merge_label));

        // ELSE branch (puede ser otro ifelse, o un bloque, o nada)
        self.code.push(format!("{}:", else_label));
        let else_result = if let Some(else_branch) = &ifelse.else_branch {
            // Si el else es otro IfElse (elif), lo procesamos recursivamente
            if let Expression::IfElse(elif) = &**else_branch {
                self.visit_ifelse(elif);
                self.last_temp.clone()
            } else {
                // Es un bloque normal
                else_branch.accept(self);
                self.last_temp.clone()
            }
        } else {
            // Si no hay else, valor por defecto
            let zero_temp = self.next_temp();
            self.code.push(format!("{} = add i32 0, 0", zero_temp));
            zero_temp
        };
        self.code.push(format!("br label %{}", merge_label));

        // MERGE
        self.code.push(format!("{}:", merge_label));
        let phi_temp = self.next_temp();
        self.code.push(format!(
            "{} = phi i32 [ {}, %{} ], [ {}, %{} ]",
            phi_temp, then_result, then_label, else_result, else_label
        ));
        self.last_temp = phi_temp;
    }

    fn visit_group(&mut self, group: &crate::ast::atoms::group::Group) {
        group.expression.accept(self);
    }

    fn visit_unary_op(&mut self, unary_op: &crate::ast::expressions::unaryoperation::UnaryOp) {
        unary_op.expr.accept(self);
        let expr_temp = self.last_temp.clone();
        let temp = self.next_temp();
        match unary_op.op {
            crate::tokens::UnaryOp::Minus(_) => {
                self.code.push(format!(
                    "{temp} = sub i32 0, {expr}",
                    temp = temp,
                    expr = expr_temp
                ));
            }
            crate::tokens::UnaryOp::Not(_) => {
                self.code.push(format!(
                    "{temp} = xor i1 {expr}, true",
                    temp = temp,
                    expr = expr_temp
                ));
            }
            _ => {
                panic!("Unsupported unary operation: {:?}", unary_op.op);
            }
        }
        self.last_temp = temp;
    }
}
