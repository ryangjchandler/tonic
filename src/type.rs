// The `Type` enumeration is the single-source of truth for type strings in Tonic.
#[derive(Debug, PartialEq)]
pub enum Type {
    String,
    Bool,
    Number,
}

impl Type {
    /// Check whether a string is a valid type string.
    pub fn valid(string: impl Into<String>) -> bool {
        matches!(string.into().as_str(), "string" | "bool" | "number")
    }

    /// Convert a type string into a valid `Type` variant.
    pub fn string(string: String) -> Self {
        match string.as_str() {
            "string" => Type::String,
            "bool" => Type::Bool,
            "number" => Type::Number,
            _ => todo!()
        }
    }
}