use crate::compiler::Scope;
use crate::{Code, Value, Op};
use crate::value::Function;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub type InternalFunction = fn (&mut VM, args: &[Value]) -> Value;

#[derive(Default, Debug)]
pub struct Frame {
    pub f_return: usize,
    environment: HashMap<String, Value>,
}

impl Frame {
    pub fn new(f_return: usize) -> Self {
        Self {
            f_return,
            ..Default::default()
        }
    }

    pub fn get(&self, name: String) -> Value {
        self.environment.get(&name).cloned().unwrap()
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.environment.insert(name, value);
    }
}

pub struct VM {
    code: Vec<Code>,
    ip: usize,

    stack: Vec<Value>,
    sp: usize,

    scopes: Vec<Scope>,
    frames: Vec<Frame>,
    fns: HashMap<String, Value>,
}

impl VM {
    pub fn new(code: Vec<Code>, scopes: Vec<Scope>) -> Self {
        Self {
            code,
            scopes,
            sp: 0,
            frames: vec![
                Frame::new(0)
            ],
            fns: HashMap::default(),
            stack: Vec::new(),
            ip: 0,
        }
    }

    pub fn push(&mut self, value: Value) -> usize {
        self.sp += 1;
        self.stack.push(value);
        self.sp
    }

    pub fn goto(&mut self, ip: usize) {
        self.ip = ip;
    }

    pub fn pop(&mut self) -> Value {
        self.sp = self.sp.saturating_sub(1);
        self.stack.pop().unwrap()
    }

    pub fn next(&mut self) {
        self.ip += 1
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

    fn current(&self) -> Code {
        self.code.get(self.ip).unwrap().clone()
    }

    pub fn run(&mut self) {
        while self.ip < self.code.len() {
            let code = self.current();

            match code {
                Code::Constant(s) => {
                    self.push(s);
                    self.next();
                },
                Code::Array(len) => {
                    let mut items: Vec<Value> = Vec::new();

                    for _ in 0..len {
                        items.push(match self.pop() {
                            Value::Array(items) => {
                                let items = items.borrow().clone();

                                Value::Array(Rc::new(RefCell::new(items)))
                            },
                            v @ _ => v,
                        });
                    }

                    self.push(Value::Array(Rc::new(RefCell::new(items))));
                    self.next();
                },
                Code::Get(s) => {
                    let value = if self.fns.contains_key(&s) {
                        self.fns.get(&s).cloned().unwrap()
                    } else {
                        self.frame().get(s)
                    };

                    self.push(value);
                    self.next();
                },
                Code::Set(s) => {
                    let value = self.pop();

                    match value {
                        Value::Function(..) => {
                            self.fns.insert(s, value);
                        },
                        _ => self.frame_mut().set(s, value),
                    };

                    self.next();
                },
                Code::Call(number_of_args) => {
                    let mut args = Vec::with_capacity(number_of_args);

                    for _ in 0..number_of_args {
                        args.push(self.pop());
                    }

                    let function = self.pop();

                    match function {
                        Value::Function(Function::Internal(_, function)) => {
                            let result = function(self, &args);

                            self.push(result);
                        },
                        Value::Function(Function::User(_, scope)) => {
                            for arg in args {
                                self.push(arg);
                            }

                            let scope = self.scopes.get(scope).unwrap().clone();

                            self.frames.push(Frame::new(self.ip));

                            self.goto(scope.start);

                            continue;
                        },
                        _ => unimplemented!()
                    };

                    self.next();
                },
                Code::Return => {
                    let value = self.pop();

                    // Exit the current frame since we're returning and don't need it anymore.
                    let frame = self.frames.pop().unwrap();

                    self.push(value);
                    self.goto(frame.f_return);

                    self.next();
                },
                Code::Op(op) => {
                    let right = self.pop();
                    let left = self.pop();

                    self.push(match op {
                        Op::Add => left + right,
                        Op::Subtract => left - right,
                        Op::Multiply => left * right,
                        Op::Divide => left / right,
                        Op::Equals => Value::Bool(left == right),
                        Op::NotEquals => Value::Bool(left != right),
                        Op::GreaterThan => Value::Bool(left > right),
                        Op::GreaterThanEquals => Value::Bool(left >= right),
                        Op::LessThan => Value::Bool(left < right),
                        Op::LessThanEquals => Value::Bool(left <= right),
                        _ => unimplemented!()
                    });

                    self.next();
                },
                Code::Jump(ip) => {
                    self.goto(ip);
                },
                Code::JumpIfElse(truthy, falsy) => {
                    let value = self.pop();

                    if value == Value::Bool(true) {
                        self.goto(truthy);
                    } else {
                        self.goto(falsy);
                    }
                },
                Code::JumpFalse(ip) => {
                    let value = self.pop();

                    if value == Value::Bool(false) {
                        self.goto(ip);
                    } else {
                        self.next();
                    }
                },
                Code::Label(_, ip) => {
                    self.goto(ip);
                },
                Code::Append => {
                    let target = self.pop();
                    let value = self.pop();

                    match target {
                        Value::Array(items) => {
                            items.borrow_mut().push(value);
                        }
                        _ => unimplemented!(),
                    };
                    
                    self.next();
                },
                _ => unimplemented!("{:?}", code),
            }
        }
    }
}