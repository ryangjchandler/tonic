use crate::{Statement, Expression, Code, Value, Function};
use std::vec::IntoIter;

#[derive(Debug, Default, Clone)]
pub struct Scope {
    pub start: usize,
    pub end: usize,
}

impl Scope {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn set_end(&mut self, end: usize) {
        self.end = end;
    }
}

#[derive(Debug)]
pub struct Compiler {
    program: IntoIter<Statement>,
    code: Vec<Code>,
    scopes: Vec<Scope>,
    scope: usize,
    breakable_ips: Vec<Vec<usize>>,
    continuable_ips: Vec<Vec<usize>>,
}

impl Compiler {

    pub fn new(program: IntoIter<Statement>) -> Self {
        Self {
            program,
            code: Vec::new(),
            scopes: vec![
                Scope::default(),
            ],
            scope: 0,
            breakable_ips: Vec::new(),
            continuable_ips: Vec::new(),
        }
    }

    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Let { identifier, initial, .. } => {
                self.compile_expression(initial);
                self.emit(Code::Set(identifier));
            },
            Statement::Function { identifier, mut parameters, body, .. } => {
                let label_position = self.emit(Code::Label(String::new(), 9999));

                let scope_index = self.enter_scope();

                parameters.reverse();
                for parameter in parameters {
                    self.emit(Code::Set(parameter.name));
                }

                for statement in body {
                    self.compile_statement(statement);
                }

                if ! matches!(self.code.last(), Some(&Code::Return)) {
                    self.emit(Code::Constant(Value::Null));
                    self.emit(Code::Return);
                }

                let function_end_position = self.leave_scope();

                // Add one here to account for the `Set` opcode.
                self.replace(label_position, Code::Label(identifier.clone(), function_end_position));

                self.emit(Code::Constant(Value::Function(Function::User(identifier.clone(), scope_index))));
                self.emit(Code::Set(identifier));
            },
            Statement::If { condition, then, otherwise } => {
                // Compile the expression.
                self.compile_expression(condition);

                let jump_if_ip = self.len();
                self.emit(Code::JumpIfElse(9999, 9999));

                // Compile the `then` block.
                let then_start = self.len();

                for statement in then {
                    self.compile_statement(statement);
                }

                let then_jump_ip = self.len();
                self.emit(Code::Jump(9999));

                // Compile the `otherwise` block.
                let otherwise_start = self.len();

                // Replace the conditional jump since we know where the blocks being now.
                self.replace(jump_if_ip, Code::JumpIfElse(then_start, otherwise_start));

                for statement in otherwise {
                    self.compile_statement(statement);
                }

                let otherwise_jump_ip = self.len();
                self.emit(Code::Jump(9999));

                let end_ip = self.len();

                self.replace(then_jump_ip, Code::Jump(end_ip));
                self.replace(otherwise_jump_ip, Code::Jump(end_ip));
            },
            Statement::While { condition, then } => {
                let pre_condition_ip = self.len();

                // Compile the expression and push to the stack.
                self.compile_expression(condition);

                // Keep track of where the falsy jump is so that we can replace it later on.
                let jump_if_false = self.emit(Code::JumpFalse(9999));

                self.enter_breakable_structure();
                self.enter_continuable_structure();

                // Compile the body of the statement.
                for statement in then {
                    self.compile_statement(statement);
                }

                // Emit a `Jump` back to the start to re-check the condition.
                self.emit(Code::Jump(pre_condition_ip));

                let after_body_ip = self.len();
                
                // This is little hacky, but `break` and `continue` statements emit jump
                // codes that need to be updated to point to the correct place.
                let breakable_ips = self.leave_breakable_structure();
                for ip in breakable_ips {
                    self.replace(ip, Code::Jump(after_body_ip));
                }

                let continuable_ips = self.leave_continuable_structure();
                for ip in continuable_ips {
                    self.replace(ip, Code::Jump(pre_condition_ip));
                }

                // Update the `JumpFalse` to jump to this position since the body
                // of the statement has ended.
                self.replace(jump_if_false, Code::JumpFalse(after_body_ip));
            },
            Statement::Break => {
                let ip = self.emit(Code::Jump(9999));

                self.breakable_ips.last_mut().unwrap().push(ip);
            },
            Statement::Continue => {
                let ip = self.emit(Code::Jump(9999));

                self.continuable_ips.last_mut().unwrap().push(ip);
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
            Expression::Bool(b) => {
                self.emit(Code::Constant(Value::Bool(b)));
            },
            Expression::Array(mut items) => {
                // Reverse the items now so we don't need to do it in the VM.
                items.reverse();

                let len = items.len();

                for item in items {
                    // Compile all of the items now.
                    self.compile_expression(item);
                }

                // Tell the VM to make a new array and push to the stack.
                self.emit(Code::Array(len));
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

    pub fn enter_breakable_structure(&mut self) {
        self.breakable_ips.push(Vec::new());
    }

    pub fn leave_breakable_structure(&mut self) -> Vec<usize> {
        self.breakable_ips.pop().unwrap()
    }

    pub fn enter_continuable_structure(&mut self) {
        self.continuable_ips.push(Vec::new());
    }

    pub fn leave_continuable_structure(&mut self) -> Vec<usize> {
        self.continuable_ips.pop().unwrap()
    }

    pub fn replace(&mut self, ip: usize, code: Code) {
        self.code[ip] = code;
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn code(&self) -> Vec<Code> {
        self.code.clone()
    }

    fn emit(&mut self, code: Code) -> usize {
        self.code.push(code);
        self.code.len() - 1
    }

    fn scope(&mut self) -> &mut Scope {
        self.scopes.get_mut(self.scope).unwrap()
    }

    fn enter_scope(&mut self) -> usize {   
        self.scope = self.scopes.len();

        self.scopes.push(Scope::new(self.code.len(), 0));

        self.scope
    }

    fn leave_scope(&mut self) -> usize {
        // End is where the end of the function definition is.
        let end = self.code.len();

        self.scope().set_end(end);
        self.scope = 0;

        end
    }

    pub fn build(&mut self) -> (Vec<Code>, Vec<Scope>) {
        while let Some(statement) = self.program.next() {
            self.compile_statement(statement);
        }

        (self.code.clone(), self.scopes.clone())
    }
}