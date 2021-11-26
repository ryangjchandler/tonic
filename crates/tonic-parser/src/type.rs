#[derive(Debug, PartialEq, Clone)]
pub struct Type(pub String);

impl From<String> for Type {
    fn from(s: String) -> Self {
        Self(s)
    }
}