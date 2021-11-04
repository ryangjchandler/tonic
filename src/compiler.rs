use crate::{Statement, Expression, Code, Value, Function};
use std::vec::IntoIter;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Scope {
    code: Vec<Code>,
}

impl Scope {
    pub fn replace(&mut self, ip: usize, code: Code) {
        self.code[ip] = code;
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn code(&self) -> Vec<Code> {
        self.code.clone()
    }
}

#[derive(Debug)]
pub struct Compiler {
    program: IntoIter<Statement>,
    scopes: Vec<Scope>,
    scope: usize,
}

impl Compiler {

    pub fn new(program: IntoIter<Statement>) -> Self {
        Self {
            program,
            scopes: vec![
                Scope::default(),
            ],
            scope: 0,
        }
    }

    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Let { identifier, initial, .. } => {
                self.compile_expression(initial);
                self.emit(Code::Set(identifier));
            },
            Statement::Function { identifier, mut parameters, body, .. } => {
                let scope_index = self.enter_scope();

                parameters.reverse();
                for parameter in parameters {
                    self.emit(Code::Set(parameter.name));
                }

                for statement in body {
                    self.compile_statement(statement);
                }

                self.emit(Code::Constant(Value::Null));
                self.emit(Code::Return);

                self.leave_scope();

                self.emit(Code::Constant(Value::Function(Function::User(identifier.clone(), scope_index))));
                self.emit(Code::Set(identifier));
            },
            Statement::If { condition, then, otherwise } => {
                // Compile the expression.
                self.compile_expression(condition);

                let jump_if_ip = self.scope().len();
                self.emit(Code::JumpIfElse(9999, 9999));

                // Compile the `then` block.
                let then_start = self.scope().len();

                for statement in then {
                    self.compile_statement(statement);
                }

                let then_jump_ip = self.scope().len();
                self.emit(Code::Jump(9999));

                // Compile the `otherwise` block.
                let otherwise_start = self.scope().len();

                // Replace the conditional jump since we know where the blocks being now.
                self.scope().replace(jump_if_ip, Code::JumpIfElse(then_start, otherwise_start));

                for statement in otherwise {
                    self.compile_statement(statement);
                }

                let otherwise_jump_ip = self.scope().len();
                self.emit(Code::Jump(9999));

                let end_ip = self.scope().len();

                self.scope().replace(then_jump_ip, Code::Jump(end_ip));
                self.scope().replace(otherwise_jump_ip, Code::Jump(end_ip));
            },
            Statement::While { condition, then } => {
                let pre_condition_ip = self.scope().len();

                // Compile the expression and push to the stack.
                self.compile_expression(condition);

                // Keep track of where the falsy jump is so that we can replace it later on.
                let jump_if_false = self.emit(Code::JumpFalse(9999));

                // Compile the body of the statement.
                for statement in then {
                    self.compile_statement(statement);
                }

                // Emit a `Jump` back to the start to re-check the condition.
                self.emit(Code::Jump(pre_condition_ip));

                let after_body_ip = self.scope().len();

                // Update the `JumpFalse` to jump to this position since the body
                // of the statement has ended.
                self.scope().replace(jump_if_false, Code::JumpFalse(after_body_ip));
            },
            Statement::Return { expression } => {
                self.compile_expression(expression);
                self.emit(Code::Return);
            },
            Statement::Expression { expression } => {
                self.compile_expression(expression);
            },
            _ => unimplemented!("statement: {:?}", statement)
        }
    }

    fn compile_expression(&mut self, expression: Expression) {
        match expression {
            Expression::String(s) => {
                self.emit(Code::Constant(Value::String(s)));
            },
            Expression::Number(n) => {
                self.emit(Code::Constant(Value::Number(n)));
            },
            Expression::Identifier(s) => {
                self.emit(Code::Get(s));
            },
            Expression::Infix(left, op, right) => {
                self.compile_expression(*left);
                self.compile_expression(*right);

                self.emit(Code::Op(op));
            },
            Expression::Call(callable, args) => {
                self.compile_expression(*callable);

                let arity = args.len();

                for arg in args {
                    self.compile_expression(arg);
                }

                self.emit(Code::Call(arity));
            },
            Expression::Assign(target, value) => {
                self.compile_expression(*value);

                match target {
                    box Expression::Identifier(i) => self.emit(Code::Set(i)),
                    _ => unimplemented!("assign to: {:?}", *target),
                };
            },
            _ => unimplemented!("{:?}", expression)
        }
    }

    fn emit(&mut self, code: Code) -> usize {
        self.scope().code.push(code);
        self.scope().code.len() - 1
    }

    fn scope(&mut self) -> &mut Scope {
        self.scopes.get_mut(self.scope).unwrap()
    }

    fn enter_scope(&mut self) -> usize {   
        self.scope = self.scopes.len();

        self.scopes.push(Scope::default());

        self.scope
    }

    fn leave_scope(&mut self) {
        self.scope = 0;
    }

    pub fn build(&mut self) -> Vec<Scope> {
        while let Some(statement) = self.program.next() {
            self.compile_statement(statement);
        }

        self.scopes.clone()
    }
}