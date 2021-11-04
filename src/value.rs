use crate::vm::InternalFunction;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Function(Function),
    Bool(bool),
    Null,
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::String(l), Value::String(r)) => l == r,
            (Value::String(l), Value::Number(r)) => l.as_str() == r.to_string().as_str(),
            (Value::Number(l), Value::String(r)) => l.to_string().as_str() == r.as_str(),
            (Value::Number(l), Value::Number(r)) => l == r,
            _ => unimplemented!()
        }
    }
}

impl Eq for Value {}

impl Value {
    pub fn to_f64(self) -> f64 {
        match self {
            Value::Number(n) => n,
            _ => unreachable!()
        }
    }
}

#[derive(Clone)]
pub enum Function {
    User(String, usize),
    Internal(InternalFunction),
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::User(name, _) => write!(f, "{}()", name),
            Function::Internal(_) => write!(f, "InternalFunction<>"),
        }
    }
}