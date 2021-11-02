use crate::{Statement, Expression};
use std::vec::IntoIter;

#[derive(Debug)]
pub struct Compiler {
    program: IntoIter<Statement>,
}

impl Compiler {

    pub fn new(program: IntoIter<Statement>) -> Self {
        Self {
            program,
        }
    }

    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            _ => unimplemented!()
        }
    }

    fn compile_expression(&mut self, expression: Expression) {
        match expression {
            _ => unimplemented!()
        }
    }

    pub fn build(&mut self) {
        while let Some(statement) = self.program.next() {
            self.compile_statement(statement);
        }
    }
}