use crate::vm::InternalFunction;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Function(Function),
    Null,
}

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