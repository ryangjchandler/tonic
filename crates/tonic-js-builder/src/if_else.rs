use crate::{Expression, Builder, builder::BuilderCallbackFunction};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct IfElse {
    condition: Expression,
    then: Builder,
    otherwise: Option<Builder>,
}

impl IfElse {
    pub fn new(condition: Expression) -> Self {
        Self {
            condition: condition,
            then: Builder::new(),
            otherwise: None,
        }
    }

    pub fn condition(&mut self, condition: Expression) -> &mut Self {
        self.condition = condition;
        self
    }

    pub fn then(&mut self, callback: BuilderCallbackFunction<Builder>) -> &mut Self {
        callback(&mut self.then);

        self
    }

    pub fn otherwise(&mut self, callback: BuilderCallbackFunction<Builder>) -> &mut Self {
        self.otherwise = Some(Builder::new());

        callback(&mut self.otherwise.as_mut().unwrap());
        
        self
    }
}

impl Display for IfElse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "if ({}) {{\n{}\n}}{}",
            self.condition,
            self.then,
            if let Some(otherwise) = &self.otherwise {
                format!(" else {{\n{}\n}}", otherwise)
            } else {
                "".to_owned()
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn new() {
        IfElse::new(Expression::from(()));
    }

    #[test]
    fn if_() {
        let mut if_else = IfElse::new(true.into());

        assert_eq!("if (true) {\n\n}", if_else.to_string().as_str());
    }

    #[test]
    fn if_body() {
        let mut if_else = IfElse::new(true.into());

        if_else.then(|body| {
            body.var(|var| {
                var.id("foo".into());
            });
        });

        assert_eq!("if (true) {\nvar foo;\n}", if_else.to_string().as_str());
    }

    #[test]
    fn else_body() {
        let mut if_else = IfElse::new(true.into());

        if_else.otherwise(|body| {
            body.var(|var| {
                var.id("foo".into());
            });
        });

        assert_eq!("if (true) {\n\n} else {\nvar foo;\n}", if_else.to_string().as_str());
    }
}