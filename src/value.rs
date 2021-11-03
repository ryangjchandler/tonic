use crate::vm::InternalFunction;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Function(Function),
    Null,
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