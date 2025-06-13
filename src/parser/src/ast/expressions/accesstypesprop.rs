use super::super::Visitable;
use super::super::Visitor;
use crate::tokens::{Identifier };
use crate::Expression;


#[derive(Debug,Clone)]
pub struct AccessTypeProp {
    pub referenced_type: Expression, // Cambiado de Identifier a Expression
    pub properties: Identifier,
    pub params: Vec<Expression>,

}

impl AccessTypeProp {
    pub fn new_expr(referenced_type: Expression, properties: Identifier, params: Vec<Expression>) -> Self {
        // Cambia el primer parámetro de Identifier a Expression
        AccessTypeProp {
            referenced_type,
            properties, 
            params,
        }
    }
     pub fn new_property_expr(referenced_type: Expression, properties: Identifier) -> Self {
        AccessTypeProp {
            referenced_type,
            properties,
            params: Vec::new(), // Sin argumentos para acceso a propiedades
        }
    }
}

impl Visitable for AccessTypeProp {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_access_type_prop(self);
    }
}