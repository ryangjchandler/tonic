use crate::{Value, Op};

#[derive(Debug, Clone)]
pub enum Code {
    Constant(Value),
    Set(String),
    Get(String),
    Op(Op),
    // `usize` here represents number of arguments that were sent across.
    // This will let us pop the values off the stack before calling the function.
    Call(usize),
    Jump(usize),
    JumpFalse(usize),
    JumpIfElse(usize, usize),
    Pop,
    Send,
    Return,
}