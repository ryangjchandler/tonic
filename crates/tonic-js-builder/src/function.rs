use crate::{Expression, Builder};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Function {
    id: String,
    parameters: Vec<Expression>,
    body: Builder,
}

impl Function {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            parameters: Vec::new(),
            body: Builder::new(),
        }
    }

    pub fn id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    pub fn parameters(&mut self, parameters: Vec<Expression>) -> &mut Self {
        self.parameters = parameters;
        self
    }

    pub fn body(&mut self, body: Builder) -> &mut Self {
        self.body = body;
        self
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "function {}({}) {{\n{}\n}}\n\n",
            self.id,
            self.parameters.clone().into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", "),
            self.body
        )
    }
}