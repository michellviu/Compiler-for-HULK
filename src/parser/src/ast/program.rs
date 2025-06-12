use super::ExpressionList;
use super::Visitable;
use super::Visitor;
// use super::Expression; // <-- Necesario para Vec<Expression>

#[derive(Debug)]
pub struct Program {
    // pub functions: Vec<Expression>,         // <-- Agrega este campo
    pub expression_list: ExpressionList,
}

impl Program {
    pub fn new(expression_list: ExpressionList) -> Self {
        Program {
            // functions: Vec::new(),          // <-- Por defecto, sin funciones
            expression_list,
        }
    }

    // pub fn new_with_functions(functions: Vec<Expression>, expression_list: ExpressionList) -> Self {
    //     Program {
    //         functions,
    //         expression_list,
    //     }
    // }
}

impl Visitable for Program {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_program(self);
    }
}