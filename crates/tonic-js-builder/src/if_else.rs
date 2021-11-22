use crate::{Expression, Builder};
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
            condition,
            then: Builder::new(),
            otherwise: None,
        }
    }

    pub fn condition(&mut self, condition: Expression) -> &mut Self {
        self.condition = condition;
        self
    }

    pub fn then(&mut self, then: Builder) -> &mut Self {
        self.then = then;
        self
    }

    pub fn otherwise(&mut self, otherwise: Builder) -> &mut Self {
        self.otherwise = Some(otherwise);
        self
    }
}

impl Display for IfElse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "if ({}) {{\n{}\n}}{}\n",
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