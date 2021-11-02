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
pub use parser::{Parser, ParserError, ParserErrorType};
use ariadne::{Report, ReportKind, Label, Source, Color};

const HELP: &'static str = "Tonic v0.1.0

usage: 
    tonic <file>";

fn main() {
    if show_help() {
        println!("{}", HELP);
        exit(0);
    }

    let (filename, source) = source();

    let ast = match Parser::new(Lexer::new(&source[..])).parse() {
        Ok(ast) => ast,
        Err(ParserError { line, span, err }) => {
            Report::build(ReportKind::Error, &filename, line)
                .with_message(match err {
                    ParserErrorType::InvalidBreakableScope => "`break` statements can only be used inside of `while` structures.".to_string(),
                    ParserErrorType::InvalidContinuableScope => "`continue` statements can only be used inside of `while` structures.".to_string(),
                    ParserErrorType::UnexpectedToken(ref token, ref expected) => {
                        if let Some(expected) = expected {
                            format!("unexpected token {}, expected {}", token, expected)
                        } else {
                            format!("unexpected token {}", token)
                        }
                    },
                    _ => unimplemented!(),
                })
                .with_code(match err {
                    ParserErrorType::InvalidBreakableScope => 032,
                    ParserErrorType::InvalidContinuableScope => 033,
                    ParserErrorType::UnexpectedToken(..) => 001,
                    _ => unimplemented!(),
                })
                .with_label(
                    Label::new((&filename, span.0 - 1 .. span.1))
                        .with_message(match err {
                            ParserErrorType::InvalidBreakableScope => "not inside of breakable structure.",
                            ParserErrorType::InvalidContinuableScope => "not inside of continuable structure.",
                            ParserErrorType::UnexpectedToken(..) => "unexpected token",
                            _ => unimplemented!()
                        })
                        .with_color(Color::Red)
                )
                .finish()
                .print((&filename, Source::from(source)))
                .unwrap();

            exit(1);
        },
    };

    dbg!(&ast);
}

fn show_help() -> bool {
    let mut args = std::env::args();

    args.len() == 1 || match args.nth(1) {
        Some(arg) => matches!(&arg[..], "--help" | "-h"),
        None => false
    }
}

fn source() -> (String, String) {
    let source = std::env::args().nth(1).unwrap();
    let path = std::path::Path::new(&source).file_name().unwrap().to_str().unwrap();

    (format!("{}", path), std::fs::read_to_string(source).unwrap())
}

fn exit(code: i32) -> ! {
    std::process::exit(code)
}