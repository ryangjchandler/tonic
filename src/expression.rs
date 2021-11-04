use crate::TokenKind;

#[derive(Debug, PartialEq)]
pub enum Expression {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<Expression>),
    Identifier(String),
    Prefix(Op, Box<Expression>),
    Infix(Box<Expression>, Op, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Assign(Box<Expression>, Box<Expression>),
    SetProperty(Box<Expression>, Box<Expression>),
    GetProperty(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

/// The `Op` enumeration is used to represent prefix, infix and other operations.
#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    Equals,
    NotEquals,
    Assign,
}

impl From<TokenKind> for Op {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Plus => Op::Add,
            TokenKind::Minus => Op::Subtract,
            TokenKind::Asterisk => Op::Multiply,
            TokenKind::Slash => Op::Divide,
            TokenKind::GreaterThan => Self::GreaterThan,
            TokenKind::LessThan => Self::LessThan,
            TokenKind::GreaterThanEquals => Self::GreaterThanEquals,
            TokenKind::LessThanEquals => Self::LessThanEquals,
            TokenKind::EqualsEquals => Self::Equals,
            TokenKind::NotEquals => Self::NotEquals,
            TokenKind::Equals => Self::Assign,
            _ => todo!()
        }
    }
}

impl From<&TokenKind> for Op {
    fn from(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Plus => Op::Add,
            TokenKind::Minus => Op::Subtract,
            TokenKind::Asterisk => Op::Multiply,
            TokenKind::Slash => Op::Divide,
            TokenKind::GreaterThan => Self::GreaterThan,
            TokenKind::LessThan => Self::LessThan,
            TokenKind::GreaterThanEquals => Self::GreaterThanEquals,
            TokenKind::LessThanEquals => Self::LessThanEquals,
            TokenKind::EqualsEquals => Self::Equals,
            TokenKind::NotEquals => Self::NotEquals,
            TokenKind::Equals => Self::Assign,
            _ => todo!()
        }
    }
}

impl Op {
    pub fn math(&self) -> bool {
        matches!(self, Self::Add | Self::Subtract | Self::Multiply | Self::Divide)
    }
}