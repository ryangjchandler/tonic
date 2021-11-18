use crate::{Statement, Expression};
use std::vec::IntoIter;

pub struct JsCompiler {
    ast: IntoIter<Statement>,
}

impl JsCompiler {
    pub fn new(ast: IntoIter<Statement>) -> Self {
        Self { ast }
    }
    
    fn statement(statement: Statement) -> String {
        match statement {
            Statement::Let { identifier, initial, .. } => {
                format!("let {} = {};\n", identifier, Self::expression(initial))
            },
            Statement::Expression { expression } => {
                format!("{};\n", Self::expression(expression))
            },
            _ => unimplemented!("js compile statement: {:?}", statement),
        }
    }

    fn expression(expression: Expression) -> String {
        match expression {
            Expression::String(s) => format!("\"{}\"", s),
            Expression::Identifier(i) => format!("{}", i),
            Expression::Call(callable, args) => {
                let args: Vec<String> = args.into_iter().map(|a| Self::expression(a)).collect();

                format!("{}({})", Self::expression(*callable), args.join(", "))
            },
            _ => unimplemented!("js compile expression: {:?}", expression)
        }
    }

    pub fn compile(&mut self) -> String {
        let mut source = String::new();

        while let Some(statement) = self.ast.next() {
            source.push_str(&Self::statement(statement));
        }

        source
    }
}