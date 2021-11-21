use crate::{Expression, Builder};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct While {
    condition: Expression,
    then: Builder,
}

impl While {
    pub fn new(condition: Expression) -> Self {
        Self {
            condition,
            then: Builder::new(),
        }
    }

    pub fn then(&mut self, then: Builder) -> &mut Self {
        self.then = then;
        self
    }
}

impl Display for While {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "while ({}) {{\n{}\n}}", self.condition, self.then)
    }
}