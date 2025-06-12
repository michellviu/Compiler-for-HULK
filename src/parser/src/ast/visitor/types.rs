#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    Boolean,
    String,
    Object,
    Custom(String), // Para type(Ident)
    Unknown,
}

impl Type {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Number" | "number" | "i32" | "f64" => Type::Number,
            "Boolean" | "boolean" | "bool" => Type::Boolean,
            "String" | "string" => Type::String,
            "Object" | "object" => Type::Object,
            _ => Type::Unknown,
        }
    }
}