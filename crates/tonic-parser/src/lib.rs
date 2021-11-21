#![feature(box_patterns)]

mod token;
mod lexer;
mod parser;
mod statement;
mod expression;
mod r#type;
mod passes;
mod js;

pub use token::{TokenKind, Token, Span};
pub use lexer::Lexer;
pub use statement::{Statement, Parameter};
pub use expression::{Expression, Op};
pub use r#type::Type;
pub use parser::{Parser, ParserError, ParserErrorType, Program};

pub fn parse(source: &str) -> Result<Vec<Statement>, ParserError> {
    let lexer = Lexer::new(source);

    let mut parser = Parser::new(lexer);
    let mut ast = parser.parse()?;

    passes::pass(&mut ast);

    Ok(ast)
}