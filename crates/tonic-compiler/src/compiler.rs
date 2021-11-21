use tonic_parser::{Statement, Expression};
use tonic_js_builder::{Builder, Var, Expression as JsExpression};
use std::vec::IntoIter;

#[derive(Debug)]
pub(crate) struct Compiler {
    ast: IntoIter<Statement>,
    builder: Builder,
}

impl Compiler {
    pub fn new(ast: IntoIter<Statement>) -> Self {
        Self {
            ast,
            builder: Builder::new(),
        }
    }

    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Let { identifier, initial, .. } => {
                let mut var = Var::new();
                
                var.id(identifier)
                    .as_let()
                    .value(self.compile_expression(initial));

                self.builder.var(var);
            },
            _ => unimplemented!("compile statement {:?}", statement),
        }
    }

    fn compile_expression(&mut self, expression: Expression) -> JsExpression {
        match expression {
            Expression::String(s) => s.into(),
            _ => unimplemented!("compile expression {:?}", expression),
        }
    }

    pub fn compile(&mut self) -> String {
        while let Some(statement) = self.ast.next() {
            self.compile_statement(statement);
        }

        self.builder.source()
    }
}