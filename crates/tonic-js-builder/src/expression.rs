use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;
use crate::Builder;

#[derive(Debug, Clone)]
pub enum Expression {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    Array(Vec<Self>),
    Object(HashMap<String, Self>),
    Index(Box<Self>, Box<Self>),
    Dot(Box<Self>, Box<Self>),
    Infix(Box<Self>, String, Box<Self>),
    Prefix(String, Box<Self>),
    Call(Box<Self>, Vec<Self>),
    Identifier(String),
    Closure(Vec<Self>, Builder),
}

impl Expression {
    pub fn number(n: f64) -> Self {
        Self::Number(n)
    }

    pub fn string(s: impl Into<String>) -> Self {
        Self::String(s.into())
    }

    pub fn bool(b: bool) -> Self {
        Self::Bool(b)
    }

    pub fn null() -> Self {
        Self::Null
    }

    pub fn identifier(id: impl Into<String>) -> Self {
        Self::Identifier(id.into())
    }

    pub fn infix(left: Expression, op: impl Into<String>, right: Expression) -> Self {
        Self::Infix(Box::new(left), op.into(), Box::new(right))
    }

    pub fn index(target: Expression, index: Expression) -> Self {
        Self::Index(Box::new(target), Box::new(index))
    }

    pub fn dot(target: Expression, property: Expression) -> Self {
        Self::Dot(Box::new(target), Box::new(property))
    }

    pub fn closure(parameters: Vec<Self>, body: Builder) -> Self {
        Self::Closure(parameters, body)
    }

    pub fn object(members: HashMap<String, Self>) -> Self {
        Self::Object(members)
    }
}

impl From<String> for Expression {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for Expression {
    fn from(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}

impl From<f64> for Expression {
    fn from(f: f64) -> Self {
        Self::Number(f)
    }
}

impl From<i64> for Expression {
    fn from(i: i64) -> Self {
        Self::Number(i as f64)
    }
}

impl From<bool> for Expression {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<()> for Expression {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl From<Vec<Self>> for Expression {
    fn from(a: Vec<Self>) -> Self {
        Self::Array(a)
    }
}

impl From<(Expression, String, Expression)> for Expression {
    fn from((left, op, right): (Expression, String, Expression)) -> Self {
        Self::Infix(Box::new(left), op, Box::new(right))
    }
}

impl From<(Box<Expression>, String, Box<Expression>)> for Expression {
    fn from((left, op, right): (Box<Expression>, String, Box<Expression>)) -> Self {
        Self::Infix(left, op, right)
    }
}

impl From<(Expression, Vec<Expression>)> for Expression {
    fn from((callable, parameters): (Expression, Vec<Expression>)) -> Self {
        Self::Call(Box::new(callable), parameters)
    }
}

impl From<(Box<Expression>, Vec<Expression>)> for Expression {
    fn from((callable, parameters): (Box<Expression>, Vec<Expression>)) -> Self {
        Self::Call(callable, parameters)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Expression::String(s) => format!(r##""{}""##, s),
            Expression::Number(n) => n.to_string(),
            Expression::Bool(b) => b.to_string(),
            Expression::Null => "null".into(),
            Expression::Array(items) => format!("[{}]", items.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ")),
            Expression::Object(members) => {
                let mut starter = String::from("{\n");

                for (key, value) in members {
                    starter.push_str(&format!("\"{}\": {},\n", key, value));
                }

                starter.push_str("\n}");
                starter
            },
            Expression::Index(target, index) => format!("{}[{}]", *target, *index),
            Expression::Dot(target, index) => format!("{}.{}", *target, *index),
            Expression::Identifier(i) => i.to_string(),
            Expression::Infix(left, op, right) => format!("{} {} {}", *left, op, *right),
            Expression::Prefix(op, right) => format!("{} {}", op, *right),
            Expression::Call(callable, parameters) => format!("{}({})", *callable, parameters.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ")),
            Expression::Closure(parameters, body) => format!("({}) => {{\n{}\n}}",
                parameters.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", "),
                body
            ),
            _ => unimplemented!()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings() {
        assert_eq!(r##""Hello!""##, Expression::from("Hello!").to_string().as_str());
    }

    #[test]
    fn numbers() {
        assert_eq!("1234", Expression::from(1234).to_string().as_str());
        assert_eq!("1234.5", Expression::from(1234.5).to_string().as_str());
    }

    #[test]
    fn bools() {
        assert_eq!("true", Expression::from(true).to_string().as_str());
        assert_eq!("false", Expression::from(false).to_string().as_str());
    }

    #[test]
    fn null() {
        assert_eq!("null", Expression::from(()).to_string().as_str());
    }

    #[test]
    fn arrays() {
        assert_eq!("[1, 2, 3]", Expression::from(vec![1.into(), 2.into(), 3.into()]).to_string().as_str());
    }

    #[test]
    fn objects() {
        let mut members = HashMap::new();
        members.insert("foo".to_owned(), Expression::String("bar".to_owned()));

        assert_eq!("{\n\"foo\": \"bar\",\n\n}", Expression::object(members).to_string().as_str());
    }

    #[test]
    fn indexes() {
        assert_eq!("[1][0]", Expression::index(
            Expression::Array(vec![Expression::Number(1.0)]), Expression::Number(0.0)
        ).to_string().as_str());
    }

    #[test]
    fn dots() {
        assert_eq!("foo.length", Expression::dot(Expression::identifier("foo"), Expression::identifier("length")).to_string().as_str());
    }

    #[test]
    fn infix() {
        assert_eq!("1 + 2", Expression::from((Expression::from(1), "+".to_string(), Expression::from(2))).to_string().as_str());
    }

    #[test]
    fn calls() {
        assert_eq!("foo()", Expression::from(
            (Expression::identifier("foo"), vec![])
        ).to_string().as_str());

        assert_eq!("foo(bar)", Expression::from(
            (Expression::identifier("foo"), vec![
                Expression::identifier("bar")
            ])
        ).to_string().as_str());
    }
}