use super::symbol_table::{SymbolInfo, SymbolTable};
use super::types::Type;
use crate::ast::visitor::visitor::Visitor;
use crate::ast::*;
use crate::tokens::*;

#[derive(Debug, Clone)]
pub struct SemanticTypeChecker {
    pub symbol_table: SymbolTable,
    pub errors: Vec<String>,
    pub input: String, // NUEVO
}

impl SemanticTypeChecker {
    pub fn new(input: String) -> Self {
        SemanticTypeChecker {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            input,
        }
    }

    pub fn check(&mut self, program: &Program) {
        program.accept(self);
    }

    fn type_of_atom(&mut self, atom: &Atom) -> Type {
        match atom {
            Atom::NumberLiteral(_) => Type::Number,
            Atom::BooleanLiteral(_) => Type::Boolean,
            Atom::StringLiteral(_) => Type::String,
            Atom::Variable(ident) => {
                if let Some(SymbolInfo::Variable { var_type }) =
                    self.symbol_table.lookup(&ident.name)
                {
                    var_type.clone()
                } else {
                    self.errors
                        .push(format!("Línea {}: Variable '{}' no declarada", 
                                     ident.position.start_line(&self.input), ident.name));
                    Type::Unknown
                }
            }
            _ => Type::Unknown,
        }
    }

    #[allow(dead_code)]
    fn get_position_info(&self, position: &Position) -> String {
        format!("Línea {}", position.start + 1)
    }

    fn infer_expr_type(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::Atom(atom) => self.type_of_atom(atom),
            Expression::FunctionCall(call) => {
                if let Some(SymbolInfo::Function { return_type, .. }) =
                    self.symbol_table.lookup(&call.funct_name.name)
                {
                    return_type.clone()
                } else {
                    self.errors
                        .push(format!("Línea {}: Función '{}' no declarada", 
                                     call.funct_name.position.start_line(&self.input), call.funct_name.name));
                    Type::Unknown
                }
            }

            Expression::BinaryOp(binop) => {
                let left_ty = self.infer_expr_type(&binop.left);
                let right_ty = self.infer_expr_type(&binop.right);
                use crate::tokens::BinOp;
                match &binop.operator {
                    BinOp::Plus(pos)
                    | BinOp::Minus(pos)
                    | BinOp::Mul(pos)
                    | BinOp::Div(pos)
                    | BinOp::Mod(pos) => {
                        if left_ty != Type::Number || right_ty != Type::Number {
                            self.errors
                                .push(format!("Línea {}: Operación aritmética requiere números", 
                                             pos.start_line(&self.input)));
                        }
                        Type::Number
                    }
                    BinOp::EqualEqual(pos)
                    | BinOp::NotEqual(pos)
                    | BinOp::Greater(pos)
                    | BinOp::Less(pos)
                    | BinOp::GreaterEqual(pos)
                    | BinOp::LessEqual(pos)
                    | BinOp::AndAnd(pos)
                    | BinOp::OrOr(pos) => {
                        if left_ty != right_ty {
                            self.errors
                                .push(format!("Línea {}: Comparación entre tipos incompatibles", 
                                             pos.start_line(&self.input)));
                        }
                        Type::Boolean
                    }
                    BinOp::ConcatString(pos) => {
                        if !(left_ty == Type::String || left_ty == Type::Number)
                            || !(right_ty == Type::String || right_ty == Type::Number)
                        {
                            self.errors
                                .push(format!("Línea {}: Concatenación requiere string o número", 
                                             pos.start_line(&self.input)));
                        }
                        Type::String
                    }
                    _ => Type::Unknown,
                }
            }
            // Agrega aquí otros casos según tu AST
            _ => Type::Unknown,
        }
    }

    fn get_expression_line(&self, expr: &Expression) -> Option<usize> {
        match expr {
            Expression::Atom(atom) => match &**atom {
                Atom::Variable(ident) => Some(ident.position.start_line(&self.input)),
                _ => None,
            },
            Expression::BinaryOp(binop) => match &binop.operator {
                BinOp::Plus(pos)
                | BinOp::Minus(pos)
                | BinOp::Mul(pos)
                | BinOp::Div(pos)
                | BinOp::Mod(pos)
                | BinOp::EqualEqual(pos)
                | BinOp::NotEqual(pos)
                | BinOp::Greater(pos)
                | BinOp::Less(pos)
                | BinOp::GreaterEqual(pos)
                | BinOp::LessEqual(pos)
                | BinOp::AndAnd(pos)
                | BinOp::OrOr(pos)
                | BinOp::ConcatString(pos)
                | BinOp::Assign(pos) => Some(pos.start_line(&self.input)),
                _ => None,
            },
            Expression::FunctionCall(call) => Some(call.funct_name.position.start_line(&self.input)),
            _ => None,
        }
    }
}

impl Visitor for SemanticTypeChecker {
    fn visit_access_type_prop(&mut self, _access: &crate::ast::expressions::accesstypesprop::AccessTypeProp) {
        
    }
    fn visit_declaration_function(&mut self, _decl: &crate::ast::expressions::declarationtypes::Declarationtypes) {
        
    }
    fn visit_instanting_types(&mut self, _inst: &crate::ast::expressions::instantiatingtypes::InstantingTypes) {
        
    }
    fn visit_type_declaration(&mut self, _decl: &crate::ast::expressions::declarationtypes::Declarationtypes) {
        
    }
    fn visit_program(&mut self, program: &Program) {
        program.expression_list.accept(self);
    }

    fn visit_expression_list(&mut self, expr_list: &ExpressionList) {
        for expr in &expr_list.expressions {
            expr.accept(self);
        }
    }

    fn visit_expression(&mut self, expr: &Expression) {
        expr.accept(self);
    }

    fn visit_block(&mut self, block: &expressions::block::Block) {
        self.symbol_table.enter_scope();
        block.expression_list.accept(self);
        self.symbol_table.exit_scope();
    }

    fn visit_print(&mut self, expr: &Expression) {
        expr.accept(self);
    }

    fn visit_literal(&mut self, _literal: &Literal) {
        // match literal {
        //     Literal::Number(_, _) => {}
        //     Literal::Bool(_, _) => {}
        //     Literal::Str(_, _) => {}
        // }
    }

    fn visit_group(&mut self, group: &atoms::group::Group) {
        group.expression.accept(self);
    }

    fn visit_for(&mut self, forr: &forr::For) {
        forr.iterable.accept(self);
        self.symbol_table.enter_scope();
        forr.var.accept(self);
        forr.body.accept(self);
        self.symbol_table.exit_scope();
    }

    fn visit_range(&mut self, start: &Expression, end: &Expression) {
        start.accept(self);
        end.accept(self);
    }

    fn visit_identifier(&mut self, _identifier: &Identifier) {}

    fn visit_unary_op(&mut self, unop: &expressions::unaryoperation::UnaryOp) {
        unop.expr.accept(self);
    }

    fn visit_function_def(&mut self, def: &expressions::functiondeclaration::FunctionDef) {
        let param_types = def.params.iter().map(|p| p.signature.clone()).collect();
        self.symbol_table.insert(
            def.name.name.clone(),
            SymbolInfo::Function {
                return_type: def.return_type.clone(),
                param_types,
            },
        );
        self.symbol_table.enter_scope();
        for param in &def.params {
            self.symbol_table.insert(
                param.name.name.clone(),
                SymbolInfo::Variable {
                    var_type: param.signature.clone(),
                },
            );
        }
        def.body.accept(self);
        self.symbol_table.exit_scope();
    }

    fn visit_function_call(&mut self, call: &expressions::functioncall::FunctionCall) {
        if let Some(SymbolInfo::Function { param_types, .. }) =
            self.symbol_table.lookup(&call.funct_name.name)
        {
            let param_types = param_types.clone();
            if param_types.len() != call.arguments.len() {
                self.errors.push(format!(
                    "Línea {}: Función '{}' espera {} argumentos, pero se pasaron {}.",
                    call.funct_name.position.start_line(&self.input),
                    call.funct_name.name,
                    param_types.len(),
                    call.arguments.len()
                ));
            }
            for (arg, expected_type) in call.arguments.iter().zip(param_types.iter()) {
                let arg_type = self.infer_expr_type(arg);
                if &arg_type != expected_type {
                    self.errors.push(format!(
                "Línea {}: El argumento tiene tipo '{:?}', pero se esperaba '{:?}' en la función '{}'.",
                call.funct_name.position.start_line(&self.input), arg_type, expected_type, call.funct_name.name
            ));
                }
            }
        } else {
            self.errors
                .push(format!("Línea {}: Función '{}' no declarada.", 
                             call.funct_name.position.start_line(&self.input), call.funct_name.name));
        }
        for arg in &call.arguments {
            arg.accept(self);
        }
    }

    fn visit_atom(&mut self, atom: &atoms::atom::Atom) {
        if let atoms::atom::Atom::Variable(ident) = atom {
            if self.symbol_table.lookup(&ident.name).is_none() {
                self.errors
                    .push(format!("Línea {}: Variable '{}' no declarada.", 
                                 ident.position.start_line(&self.input), ident.name));
            }
        }
    }

    fn visit_assignment(&mut self, assign: &expressions::letin::Assignment) {
        if let atoms::atom::Atom::Variable(ident) = &assign.variable {
            let assigned_type = self.infer_expr_type(&assign.body);
            self.symbol_table.insert(
                ident.name.clone(),
                SymbolInfo::Variable {
                    var_type: assigned_type,
                },
            );
        }
        assign.body.accept(self);
    }

    fn visit_letin(&mut self, letin: &expressions::letin::LetIn) {
        self.symbol_table.enter_scope();
        for assign in &letin.bindings {
            assign.accept(self);
        }
        letin.body.accept(self);
        self.symbol_table.exit_scope();
    }

    fn visit_binary_op(&mut self, binop: &crate::ast::expressions::binoperation::BinaryOp) {
        binop.left.accept(self);
        binop.right.accept(self);
        use crate::tokens::BinOp;
        let left_ty = self.infer_expr_type(&binop.left);
        let right_ty = self.infer_expr_type(&binop.right);

        match &binop.operator {
            BinOp::Plus(pos) | BinOp::Minus(pos) | BinOp::Mul(pos) | BinOp::Div(pos) | BinOp::Mod(pos) => {
                if left_ty != Type::Number || right_ty != Type::Number {
                    self.errors
                        .push(format!("Línea {}: Operación aritmética requiere números", 
                                     pos.start_line(&self.input)));
                }
            }
            BinOp::EqualEqual(pos)
            | BinOp::NotEqual(pos)
            | BinOp::Greater(pos)
            | BinOp::Less(pos)
            | BinOp::GreaterEqual(pos)
            | BinOp::LessEqual(pos) => {
                if left_ty != right_ty {
                    self.errors
                        .push(format!("Línea {}: Comparación entre tipos incompatibles", 
                                     pos.start_line(&self.input)));
                }
            }
            BinOp::AndAnd(pos) | BinOp::OrOr(pos) => {
                if left_ty != Type::Boolean || right_ty != Type::Boolean {
                    self.errors
                        .push(format!("Línea {}: Operador lógico requiere booleanos", 
                                     pos.start_line(&self.input)));
                }
            }
            BinOp::ConcatString(pos) => {
                if !(left_ty == Type::String || left_ty == Type::Number)
                    || !(right_ty == Type::String || right_ty == Type::Number)
                {
                    self.errors
                        .push(format!("Línea {}: Concatenación requiere string o número", 
                                     pos.start_line(&self.input)));
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
            let line = self.get_expression_line(&ifelse.condition)
                .map(|l| format!("Línea {}: ", l))
                .unwrap_or_default();
            self.errors
                .push(format!("{}Condición de if debe ser booleana", line));
        }
        ifelse.then_branch.accept(self);
        for (_, cond, branch) in &ifelse.elif_branches {
            cond.accept(self);
            let t = self.infer_expr_type(cond);
            if t != Type::Boolean {
                let line = self.get_expression_line(cond)
                    .map(|l| format!("Línea {}: ", l))
                    .unwrap_or_default();
                self.errors
                    .push(format!("{}Condición de elif debe ser booleana", line));
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
            self.errors
                .push("Condición de while debe ser booleana".to_string());
        }
        whilee.body.accept(self);
    }

    // Implementa los demás métodos igual que antes, usando self.errors para reportar problemas
    // ...
}
