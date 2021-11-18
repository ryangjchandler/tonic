use crate::{Statement, Expression, Op};
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
            Statement::Function { identifier, parameters, body, .. } => {
                format!(
                    "function {}({}) {{\n{}}}",
                    identifier,
                    parameters.into_iter().map(|p| p.name).collect::<Vec<String>>().join(", "),
                    body.into_iter().map(|b| Self::statement(b)).collect::<Vec<String>>().join("\t\n")
                )
            },
            Statement::If { condition, then, .. } => {
                format!(
                    "if ({}) {{\n{}\n}}",
                    Self::expression(condition),
                    then.into_iter().map(|b| Self::statement(b)).collect::<Vec<String>>().join("\t"),
                )
            },
            Statement::Return { expression } => {
                format!("return {};", Self::expression(expression))
            },
            _ => unimplemented!("js compile statement: {:?}", statement),
        }
    }

    fn expression(expression: Expression) -> String {
        match expression {
            Expression::String(s) => format!("\"{}\"", s),
            Expression::Number(n) => format!("{}", n),
            Expression::Identifier(i) => format!("{}", i),
            Expression::Bool(b) => format!("{}", b),
            Expression::Array(items) => {
                format!("[{}]", items.into_iter().map(|i| Self::expression(i)).collect::<Vec<String>>().join(", "))
            },
            Expression::Call(callable, args) => {
                let args: Vec<String> = args.into_iter().map(|a| Self::expression(a)).collect();

                format!("{}({})", Self::expression(*callable), args.join(", "))
            },
            Expression::Prefix(op, right) => {
                format!("{}{}", match op {
                    Op::Subtract => "-",
                    _ => unreachable!()
                }, Self::expression(*right))
            },
            Expression::Infix(left, op, right) => {
                format!("{} {} {}", Self::expression(*left), match op {
                    Op::LessThan => "<",
                    Op::GreaterThan => ">",
                    Op::LessThanEquals => "<=",
                    Op::GreaterThanEquals => ">=",
                    Op::Add => "+",
                    Op::Subtract => "-",
                    Op::Multiply => "*",
                    Op::Divide => "/",
                    Op::Equals => "===",
                    Op::NotEquals => "!==",
                    _ => unimplemented!()
                }, Self::expression(*right))
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