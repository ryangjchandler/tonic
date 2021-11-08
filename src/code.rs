use crate::{Value, Op, Function};

#[derive(Debug, Clone)]
pub enum Code {
    // name of the label (function name), position to jump to to skip over function
    Label(String, usize),
    Constant(Value),
    // A special code used to capture the current frame's environment and store alongside the closure.
    Closure(Function),
    Array(usize),
    Set(String),
    Get(String),
    SetProperty,
    GetProperty,
    Append,
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