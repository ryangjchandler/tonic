use crate::compiler::Scope;
use crate::{Code, Value};
use crate::value::Function;
use std::collections::HashMap;

pub type InternalFunction = fn (&mut VM, args: &[Value]) -> Value;

pub struct Frame {
    return_scope: usize,
}

pub struct VM {
    scopes: Vec<Scope>,
    scope: usize,
    frames: Vec<Frame>,
    fns: HashMap<String, Value>,
}

impl VM {
    pub fn new(scopes: Vec<Scope>) -> Self {
        Self {
            scopes,
            scope: 0,
            frames: vec![],
            fns: HashMap::default(),
        }
    }

    fn scope(&self) -> &Scope {
        self.scopes.get(self.scope).unwrap()
    }

    fn scope_mut(&mut self) -> &mut Scope {
        self.scopes.get_mut(self.scope).unwrap()
    }

    pub fn add_function(&mut self, name: &str, callback: InternalFunction) {
        self.fns.insert(name.into(), Value::Function(Function::Internal(callback)));
    }

    pub fn run(&mut self) {
        while self.scope().runnable() {
            let code = self.scope().current();

            match code {
                Code::Constant(s) => {
                    self.scope_mut().push(s);
                    self.scope_mut().next();
                },
                Code::Get(s) => {
                    let value = if self.fns.contains_key(&s) {
                        self.fns.get(&s).cloned().unwrap()
                    } else {
                        self.scope().get(s)
                    };

                    self.scope_mut().push(value);
                    self.scope_mut().next();
                },
                Code::Set(s) => {
                    let value = self.scope_mut().pop();

                    match value {
                        _ => self.scope_mut().set(s, value),
                    }

                    self.scope_mut().next();
                },
                Code::Call(number_of_args) => {
                    let mut args = Vec::with_capacity(number_of_args);

                    for _ in 0..number_of_args {
                        args.push(self.scope_mut().pop());
                    }

                    let function = self.scope_mut().pop();

                    match function {
                        Value::Function(Function::Internal(function)) => {
                            let result = function(self, &args);

                            self.scope_mut().push(result);
                        },
                        Value::Function(Function::User(_, scope)) => {
                            self.frames.push(Frame { return_scope: self.scope });
                            self.scope = scope;

                            for arg in args {
                                self.scope_mut().push(arg);
                            }

                            continue;
                        },
                        _ => unimplemented!()
                    };

                    self.scope_mut().next();
                },
                Code::Return => {
                    let value = self.scope_mut().pop();
                    let Frame { return_scope } = self.frames.pop().unwrap();

                    self.scope = return_scope;

                    self.scope_mut().push(value);
                    self.scope_mut().next();
                },
                _ => unimplemented!("{:?}", code),
            }
        }
    }
}