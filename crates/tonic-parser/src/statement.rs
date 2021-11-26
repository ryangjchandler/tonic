use crate::{Expression, Type};

/// The main type of `Node` in Tonic. Every line in the source code will eventually be parsed into
/// a `Statement`, including arbitrary expressions.
/// 
/// This enum is used to describe the most common structures in the Tonic language. It does not hold any
/// information about the position of the node, that is the responsibility of `Node`.
#[derive(Debug, PartialEq)]
pub enum Statement {
    Let {
        identifier: String,
        initial: Expression,
    },
    Function {
        identifier: String,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Statement>,
    },
    If {
        condition: Expression,
        then: Vec<Statement>,
        otherwise: Vec<Statement>,
    },
    While {
        condition: Expression,
        then: Vec<Statement>,
    },
    Return {
        expression: Expression,
    },
    Expression {
        expression: Expression,
    },
    Use {
        module: String,
        imports: Vec<String>,
    },
    Break,
    Continue,
}

/// The `Parameter` struct is used to represent a function parameter.
/// 
/// It stores information about the name of the parameter and the expected type of the parameter.
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
    pub r#type: Option<Type>,
}

impl Parameter {
    pub fn new(name: impl Into<String>, r#type: Option<Type>) -> Self {
        Self { name: name.into(), r#type }
    }
}