use crate::{Var, Expression, Function};
use std::fmt::{Result, Formatter, Display};

pub type BuilderCallbackFunction<T> = fn (&mut T);

#[derive(Debug)]
pub struct Builder {
    source: String,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            source: String::default(),
        }
    }

    pub fn var(&mut self, builder: BuilderCallbackFunction<Var>) -> &mut Self {
        let mut var = Var::new();

        builder(&mut var);

        self.source.push_str(&var.to_string());
        self
    }

    pub fn function(&mut self, builder: BuilderCallbackFunction<Function>) -> &mut Self {
        let mut function = Function::new();

        builder(&mut function);
        
        self.source.push_str(&function.to_string());
        self
    }

    pub fn expression(&mut self, expression: Expression) -> &mut Self {
        self.source.push_str(&expression.to_string());
        self.source.push(';');

        self
    }
}

impl Display for Builder {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_() {
        let mut builder = Builder::new();

        builder
            .var(|var| {
                var
                    .id("foo".into())
                    .value(123.into());
            });

        assert_eq!(builder.to_string(), String::from("var foo = 123;"))
    }

    #[test]
    fn expression_() {
        let mut builder = Builder::new();

        builder
            .expression(().into());

        assert_eq!(builder.to_string(), String::from("null;"));
    }
}