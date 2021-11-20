use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Expression {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    Array(Vec<Self>),
}

impl Expression {
    pub fn number(n: f64) -> Self {
        Self::Number(n)
    }

    pub fn string(s: String) -> Self {
        Self::String(s)
    }

    pub fn bool(b: bool) -> Self {
        Self::Bool(b)
    }

    pub fn null() -> Self {
        Self::Null
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

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Expression::String(s) => format!(r##""{}""##, s),
            Expression::Number(n) => n.to_string(),
            Expression::Bool(b) => b.to_string(),
            Expression::Null => "null".into(),
            Expression::Array(items) => format!("[{}]", items.into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ")),
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
}