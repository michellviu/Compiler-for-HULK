use super::super::Visitable;
use super::super::Visitor;
use crate::tokens::{Identifier };
use crate::Expression;


#[derive(Debug)]
pub struct AccessTypeProp {
    pub referenced_type: Identifier,
    pub properties: Identifier,
    pub param: Vec<Expression>,

}

impl AccessTypeProp {
    pub fn new_expr(referenced_type: Identifier, properties: Identifier, params: Vec<Expression> ) -> Self {
        AccessTypeProp {
            referenced_type,
            properties,
            params,
        }
    }
}

impl Visitable for AccessTypeProp {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_access_type_prop(self);
    }
}