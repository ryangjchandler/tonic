use tonic_parser::{Statement, Expression, Op};
use tonic_js_builder::{Builder, Var, While, IfElse, Function, Expression as JsExpression};
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
            Statement::Return { expression } => {
                let expression = self.compile_expression(expression);
                self.builder.return_(Some(expression));
            },
            Statement::While { condition, then } => {
                let condition = self.compile_expression(condition);
                let mut then = Compiler::new(then.into_iter());
                then.compile();

                let mut while_ = While::new(condition);
                while_.then(then.builder());

                self.builder.while_loop(while_);
            },
            Statement::If { condition, then, otherwise } => {
                let condition = self.compile_expression(condition);

                let mut then = Compiler::new(then.into_iter());
                then.compile();

                let mut otherwise = Compiler::new(otherwise.into_iter());
                otherwise.compile();

                let mut if_ = IfElse::new(condition);
                if_
                    .then(then.builder())
                    .otherwise(otherwise.builder());

                self.builder.conditional(if_);
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
            Expression::Number(n) => n.into(),
            Expression::Identifier(i) => JsExpression::identifier(i),
            Expression::Infix(left, op, right) => {
                JsExpression::from((
                    self.compile_expression(*left),
                    (match op {
                        Op::GreaterThan => ">",
                        Op::LessThan => "<",
                        Op::GreaterThanEquals => ">=",
                        Op::LessThanEquals => "<=",
                        Op::Add => "+",
                        Op::Subtract => "-",
                        Op::Multiply => "*",
                        Op::Divide => "/",
                        Op::Equals => "===",
                        Op::NotEquals => "!==",
                        _ => unimplemented!(),
                    }).to_string(),
                    self.compile_expression(*right),
                ))
            },
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