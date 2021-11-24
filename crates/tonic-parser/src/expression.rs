use crate::TokenKind;
use crate::{Parameter, Statement};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Expression {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<Expression>),
    Map(HashMap<String, Expression>),
    Identifier(String),
    Prefix(Op, Box<Expression>),
    Infix(Box<Expression>, Op, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Assign(Box<Expression>, Box<Expression>),
    Index(Box<Expression>, Option<Box<Expression>>),
    Dot(Box<Expression>, Box<Expression>),
    Closure(Vec<Parameter>, Vec<Statement>),
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
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    Not,
    And,
    Or,
    Mod,
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
            TokenKind::Not => Self::Not,
            TokenKind::And => Self::And,
            TokenKind::Or => Self::Or,
            TokenKind::Percent => Self::Mod,
            TokenKind::PlusEquals => Self::AddAssign,
            TokenKind::MinusEquals => Self::SubtractAssign,
            TokenKind::AsteriskEquals => Self::MultiplyAssign,
            TokenKind::SlashEquals => Self::DivideAssign,
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
            TokenKind::Not => Self::Not,
            TokenKind::And => Self::And,
            TokenKind::Or => Self::Or,
            TokenKind::Percent => Self::Mod,
            TokenKind::PlusEquals => Self::AddAssign,
            TokenKind::MinusEquals => Self::SubtractAssign,
            TokenKind::AsteriskEquals => Self::MultiplyAssign,
            TokenKind::SlashEquals => Self::DivideAssign,
            _ => todo!()
        }
    }
}

impl Op {
    pub fn math(&self) -> bool {
        matches!(self, Self::Add | Self::Subtract | Self::Multiply | Self::Divide)
    }
}