use crate::{Var, Expression, Function, IfElse, While};
use std::fmt::{Result, Formatter, Display};

pub type BuilderCallbackFunction<T> = fn (&mut T);

#[derive(Debug, Clone)]
pub struct Builder {
    source: String,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            source: String::default(),
        }
    }

    pub fn var(&mut self, var: Var) -> &mut Self {
        self.source.push_str(&var.to_string());
        self
    }

    pub fn function(&mut self, function: Function) -> &mut Self {        
        self.source.push_str(&function.to_string());
        self
    }

    pub fn conditional(&mut self, if_else: IfElse) -> &mut Self {
        self.source.push_str(&if_else.to_string());
        self
    }

    pub fn while_loop(&mut self, while_: While) -> &mut Self {
        self.source.push_str(&while_.to_string());
        self
    }

    pub fn return_(&mut self, expression: Option<Expression>) -> &mut Self {
        self.source.push_str("return");

        if let Some(expression) = expression {
            self.source.push_str(" ");
            self.source.push_str(&expression.to_string());
        }

        self.source.push_str(";");

        self
    }

    pub fn break_(&mut self) -> &mut Self {
        self.source.push_str("break;");
        self
    }

    pub fn continue_(&mut self) -> &mut Self {
        self.source.push_str("continue;");
        self
    }

    pub fn expression(&mut self, expression: Expression) -> &mut Self {
        self.source.push_str(&expression.to_string());
        self.source.push(';');

        self
    }

    pub fn source(&self) -> String {
        self.source.clone()
    }
}

impl Display for Builder {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.source)
    }
}