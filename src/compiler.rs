use crate::{Statement, Expression, Code, Value};
use std::vec::IntoIter;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Scope {
    ip: usize,
    code: Vec<Code>,
    stack: Vec<Value>,
    sp: usize,
    environment: HashMap<String, Value>,
}

impl Scope {
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

#[derive(Debug)]
pub struct Compiler {
    program: IntoIter<Statement>,
    scopes: Vec<Scope>,
}

impl Compiler {

    pub fn new(program: IntoIter<Statement>) -> Self {
        Self {
            program,
            scopes: vec![
                Scope::default(),
            ],
        }
    }

    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Let { identifier, initial, .. } => {
                self.compile_expression(initial);
                self.emit(Code::Set(identifier));
            },
            Statement::Expression { expression } => {
                self.compile_expression(expression);
            },
            _ => unimplemented!()
        }
    }

    fn compile_expression(&mut self, expression: Expression) {
        match expression {
            Expression::String(s) => {
                self.emit(Code::Constant(Value::String(s)));
            },
            Expression::Identifier(s) => {
                self.emit(Code::Get(s));
            },
            Expression::Call(callable, args) => {
                self.compile_expression(*callable);

                let arity = args.len();

                for arg in args {
                    self.compile_expression(arg);
                }

                self.emit(Code::Call(arity));
            },
            _ => unimplemented!("{:?}", expression)
        }
    }

    fn emit(&mut self, code: Code) -> usize {
        self.scope().code.push(code);
        self.scope().code.len()
    }

    fn scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    pub fn build(&mut self) -> Vec<Scope> {
        while let Some(statement) = self.program.next() {
            self.compile_statement(statement);
        }

        self.scopes.clone()
    }
}