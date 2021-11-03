use crate::Value;

#[derive(Debug, Clone)]
pub enum Code {
    Constant(Value),
    Set(String),
    Get(String),
    // `usize` here represents number of arguments that were sent across.
    // This will let us pop the values off the stack before calling the function.
    Call(usize),
    Pop,
    Send,
    Return,
}