use super::super::expressions::declarationtypes::Declarationtypes;
use super::super::expressions::functiondeclaration::FunctionDef;
use super::types::Type;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub properties: Vec<(String, Type)>,     // nombre
    pub methods: Vec<(String, FunctionDef)>, // nombre y definición de cada método
}

#[derive(Debug, Clone)]
pub enum SymbolInfo {
    Variable {
        var_type: Type,
    },
    Function {
        return_type: Type,
        param_types: Vec<Type>,
    },
    Type {
        type_info: TypeInfo,
    },
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    scopes: Vec<HashMap<String, SymbolInfo>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, name: String, info: SymbolInfo) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, info);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    pub fn insert_type(
        &mut self,
        name: String,
        properties: Vec<(String, Type)>,
        methods: Vec<(String, FunctionDef)>,
    ) {
        let type_info = TypeInfo {
            name: name.clone(),
            properties,
            methods,
        };
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, SymbolInfo::Type { type_info });
        }
    }

    pub fn has_type(&self, name: &str) -> bool {
        self.lookup(name)
            .map_or(false, |info| matches!(info, SymbolInfo::Type { .. }))
    }

    pub fn get_type(&self, name: &str) -> Option<&TypeInfo> {
        match self.lookup(name) {
            Some(SymbolInfo::Type { type_info }) => Some(type_info),
            _ => None,
        }
    }

    pub fn has_property(&self, type_name: &str, prop_name: &str) -> bool {
        self.get_type(type_name).map_or(false, |type_info| {
            type_info.properties.iter().any(|n| n.0 == prop_name)
        })
    }

    pub fn has_method(&self, type_name: &str, method_name: &str) -> bool {
        self.get_type(type_name).map_or(false, |type_info| {
            type_info.methods.iter().any(|(n, _)| n == method_name)
        })
    }

    pub fn property_index(&self, type_name: &str, prop_name: &str) -> usize {
        self.get_type(type_name)
            .and_then(|type_info| type_info.properties.iter().position(|n| n.0 == prop_name))
            .unwrap_or(0)
    }

    pub fn property_llvm_type(&self, type_name: &str, prop_name: &str) -> &'static str {
        self.get_type(type_name)
            .and_then(|type_info| type_info.properties.iter().find(|(n, _)| n == prop_name))
            .map(|(_, t)| match t {
                Type::Number => "i32",
                Type::Boolean => "i1",
                Type::String => "i8*",
                _ => "i8*",
            })
            .unwrap_or("i8*")
    }
}
