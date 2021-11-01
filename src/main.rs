mod token;
mod lexer;
mod parser;
mod statement;
mod expression;
mod r#type;

pub use token::{TokenKind, Token, Span};
pub use lexer::Lexer;
pub use statement::{Statement, Parameter};
pub use expression::{Expression, Op};
pub use r#type::Type;
pub use parser::Parser;

const HELP: &'static str = "Tonic v0.1.0

usage: 
    tonic <file>";

fn main() {
    if show_help() {
        println!("{}", HELP);
        exit(0);
    }

    let source = source();

    let mut parser = Parser::new(Lexer::new(&source[..]));
    let ast = parser.parse();

    println!("Program:");

    dbg!(&ast);
}

fn show_help() -> bool {
    let mut args = std::env::args();

    args.len() == 1 || match args.nth(1) {
        Some(arg) => matches!(&arg[..], "--help" | "-h"),
        None => false
    }
}

fn source() -> String {
    let source = std::env::args().nth(1).unwrap();

    std::fs::read_to_string(source).unwrap()
}

fn exit(code: i32) -> ! {
    std::process::exit(code)
}