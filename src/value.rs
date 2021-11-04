use crate::vm::InternalFunction;
use std::cmp::Ordering;

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
            (Value::Bool(l), Value::Bool(r)) => l == r,
            _ => unimplemented!()
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Value::String(l), Value::String(r)) => Some(l.cmp(r)),
            (Value::String(l), Value::Number(r)) => {
                let l_str = l.as_str();
                let r_string = r.to_string();
                let r_str = r_string.as_str();

                Some(l_str.cmp(r_str))
            },
            (Value::Number(l), Value::String(r)) => {
                let l_string = l.to_string();
                let l_str = l_string.as_str();

                let r_str = r.as_str();

                Some(l_str.cmp(r_str))
            },
            (Value::Number(l), Value::Number(r)) => if l < r { Some(Ordering::Less) } else if l > r { Some(Ordering::Greater) } else { Some(Ordering::Equal) },
            _ => unimplemented!()
        }
    }
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
    Internal(&'static str, InternalFunction),
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::User(name, _) => write!(f, "{}()", name),
            Function::Internal(name, _) => write!(f, "InternalFunction<{}>", name),
        }
    }
}