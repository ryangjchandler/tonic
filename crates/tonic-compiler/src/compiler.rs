use tonic_parser::{Statement, Expression};
use tonic_js_builder::{Builder, Var, Function, Expression as JsExpression};
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
            Statement::Function { identifier, parameters, body, .. } => {
                let mut function = Function::new();
                
                let mut body = Compiler::new(body.into_iter());
                body.compile();

                function
                    .id(identifier)
                    .parameters(
                        parameters.into_iter().map(|p| JsExpression::identifier(p.name)).collect::<Vec<JsExpression>>()
                    )
                    .body(body.builder());

                self.builder.function(function);
            },
            Statement::Expression { expression } => {
                let expression = self.compile_expression(expression);

                self.builder.expression(expression);
            },
            _ => unimplemented!("compile statement {:?}", statement),
        }
    }

    fn compile_expression(&mut self, expression: Expression) -> JsExpression {
        match expression {
            Expression::String(s) => s.into(),
            Expression::Identifier(i) => JsExpression::identifier(i),
            Expression::Call(callable, args) => {
                JsExpression::Call(
                    Box::new(self.compile_expression(*callable)),
                    args.into_iter().map(|a| self.compile_expression(a)).collect::<Vec<JsExpression>>()
                )
            },
            _ => unimplemented!("compile expression {:?}", expression),
        }
    }

    pub fn compile(&mut self) -> String {
        while let Some(statement) = self.ast.next() {
            self.compile_statement(statement);
        }

        self.builder.source()
    }

    pub fn builder(&self) -> Builder {
        self.builder.clone()
    }
}