use crate::Expression;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Var {
    id: String,
    value: Option<Expression>,
    m_const: bool,
    m_let: bool,
}

impl Var {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            value: None,
            m_const: false,
            m_let: false,
        }
    }

    pub fn as_let(&mut self) -> &mut Self {
        self.m_let = true;
        self
    }

    pub fn as_const(&mut self) -> &mut Self {
        self.m_const = true;
        self.m_let = false;
        self
    }

    pub fn id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    pub fn value(&mut self, expression: Expression) -> &mut Self {
        self.value = Some(expression);
        self
    }

    fn keyword(&self) -> String {
        String::from(if self.m_let {
            "let"
        } else if self.m_const {
            "const"
        } else {
            "var"
        })
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}{};", self.keyword(), self.id, if let Some(expression) = &self.value {
            format!(" = {}", expression)
        } else {
            String::from("")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        Var::new();
    }

    #[test]
    fn set_id() {
        let mut var = Var::new();
        
        var.id("foo".to_owned());

        assert_eq!(var.to_string(), String::from("var foo;"));
    }

    #[test]
    fn set_const() {
        let mut var = Var::new();
        
        var
            .id("foo".to_owned())
            .as_const();

        assert_eq!(var.to_string(), String::from("const foo;"));
    }

    #[test]
    fn set_let() {
        let mut var = Var::new();
        
        var
            .id("foo".to_owned())
            .as_let();

        assert_eq!(var.to_string(), String::from("let foo;"));
    }

    #[test]
    fn with_value() {
        let mut var = Var::new();
        
        var
            .id("foo".into())
            .value("Hello!".into());

        assert_eq!(var.to_string(), String::from(r##"var foo = "Hello!";"##));
    }
}