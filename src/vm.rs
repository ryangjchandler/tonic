use crate::compiler::Scope;
use crate::{Code, Value, Op};
use crate::value::Function;
use std::collections::HashMap;

pub type InternalFunction = fn (&mut VM, args: &[Value]) -> Value;

#[derive(Default, Debug)]
pub struct Frame {
    code: Vec<Code>,
    ip: usize,
    stack: Vec<Value>,
    sp: usize,
    environment: HashMap<String, Value>,
}

impl Frame {
    pub fn new(code: Vec<Code>) -> Self {
        Self {
            code,
            ..Default::default()
        }
    }

    pub fn goto(&mut self, ip: usize) {
        self.ip = ip;
    }

    pub fn runnable(&self) -> bool {
        self.ip < self.code.len()
    }

    pub fn current(&self) -> Code {
        self.code.get(self.ip).unwrap().clone()
    }

    pub fn push(&mut self, value: Value) -> usize {
        self.sp += 1;
        self.stack.push(value);
        self.sp
    }

    pub fn pop(&mut self) -> Value {
        self.sp = self.sp.saturating_sub(1);
        self.stack.pop().unwrap()
    }

    pub fn get(&self, name: String) -> Value {
        self.environment.get(&name).cloned().unwrap()
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.environment.insert(name, value);
    }

    pub fn next(&mut self) {
        self.ip += 1
    }
}

pub struct VM {
    scopes: Vec<Scope>,
    frames: Vec<Frame>,
    fns: HashMap<String, Value>,
}

impl VM {
    pub fn new(scopes: Vec<Scope>) -> Self {
        let entry = scopes.first().unwrap().code();

        Self {
            scopes,
            frames: vec![
                Frame::new(entry)
            ],
            fns: HashMap::default(),
        }
    }

    pub fn add_function(&mut self, name: &'static str, callback: InternalFunction) {
        self.fns.insert(name.into(), Value::Function(Function::Internal(name, callback)));
    }

    fn frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    fn frame(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    pub fn run(&mut self) {
        while self.frame().runnable() {
            let code = self.frame().current();

            match code {
                Code::Constant(s) => {
                    self.frame_mut().push(s);
                    self.frame_mut().next();
                },
                Code::Get(s) => {
                    let value = if self.fns.contains_key(&s) {
                        self.fns.get(&s).cloned().unwrap()
                    } else {
                        self.frame().get(s)
                    };

                    self.frame_mut().push(value);
                    self.frame_mut().next();
                },
                Code::Set(s) => {
                    let value = self.frame_mut().pop();

                    match value {
                        Value::Function(..) => {
                            self.fns.insert(s, value);
                        }
                        _ => self.frame_mut().set(s, value),
                    }

                    self.frame_mut().next();
                },
                Code::Call(number_of_args) => {
                    let mut args = Vec::with_capacity(number_of_args);

                    for _ in 0..number_of_args {
                        args.push(self.frame_mut().pop());
                    }

                    let function = self.frame_mut().pop();

                    match function {
                        Value::Function(Function::Internal(_, function)) => {
                            let result = function(self, &args);

                            self.frame_mut().push(result);
                        },
                        Value::Function(Function::User(_, scope)) => {
                            let scope = self.scopes.get(scope).unwrap();

                            self.frames.push(Frame::new(scope.code()));

                            for arg in args {
                                self.frame_mut().push(arg);
                            }

                            continue;
                        },
                        _ => unimplemented!()
                    };

                    self.frame_mut().next();
                },
                Code::Return => {
                    let value = self.frame_mut().pop();

                    // Exit the current frame since we're returning and don't need it anymore.
                    self.frames.pop();

                    self.frame_mut().push(value);
                    self.frame_mut().next();
                },
                Code::Op(op) => {
                    let right = self.frame_mut().pop();
                    let left = self.frame_mut().pop();

                    match (left.clone(), right.clone()) {
                        (Value::Number(l), Value::Number(r)) if op.math() => {
                            self.frame_mut().push(Value::Number(match op {
                                Op::Add => l + r,
                                Op::Subtract => l - r,
                                Op::Multiply => l * r,
                                Op::Divide => l / r,
                                _ => unreachable!()
                            }));
                        },
                        _ => {
                            self.frame_mut().push(match op {
                                Op::Equals => Value::Bool(left == right),
                                Op::NotEquals => Value::Bool(left != right),
                                Op::GreaterThan => Value::Bool(left > right),
                                Op::LessThan => Value::Bool(left < right),
                                Op::GreaterThanEquals => Value::Bool(left >= right),
                                Op::LessThanEquals => Value::Bool(left <= right),
                                _ => unimplemented!("op: {:?}, left: {:?}, right: {:?}", op, left, right),
                            });
                        }
                    };

                    self.frame_mut().next();
                },
                Code::Jump(ip) => {
                    self.frame_mut().goto(ip);
                },
                Code::JumpIfElse(truthy, falsy) => {
                    let value = self.frame_mut().pop();

                    if value == Value::Bool(true) {
                        self.frame_mut().goto(truthy);
                    } else {
                        self.frame_mut().goto(falsy);
                    }
                },
                Code::JumpFalse(ip) => {
                    let value = self.frame_mut().pop();

                    if value == Value::Bool(false) {
                        self.frame_mut().goto(ip);
                    } else {
                        self.frame_mut().next();
                    }
                }
                _ => unimplemented!("{:?}", code),
            }
        }
    }
}