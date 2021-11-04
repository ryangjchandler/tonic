#![feature(box_patterns)]

mod token;
mod lexer;
mod parser;
mod statement;
mod expression;
mod r#type;
mod compiler;
mod value;
mod code;
mod vm;

pub use token::{TokenKind, Token, Span};
pub use lexer::Lexer;
pub use statement::{Statement, Parameter};
pub use expression::{Expression, Op};
pub use r#type::Type;
pub use parser::{Parser, ParserError, ParserErrorType, Program};
pub use compiler::Compiler;
pub use value::{Value, Function};
pub use code::Code;
use ariadne::{Report, ReportKind, Label, Source, Color};

const HELP: &str = "Tonic v0.1.0

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
                    ParserErrorType::ExpectedIdentifier => "expected a valid identifier".to_string(),
                    _ => unimplemented!(),
                })
                .with_code(match err {
                    ParserErrorType::InvalidBreakableScope => 32,
                    ParserErrorType::InvalidContinuableScope => 33,
                    ParserErrorType::UnexpectedToken(..) => 1,
                    ParserErrorType::ExpectedIdentifier => 2,
                    _ => unimplemented!(),
                })
                .with_label(
                    Label::new((&filename, span.0.saturating_sub(1) .. span.1))
                        .with_message(match err {
                            ParserErrorType::InvalidBreakableScope => "not inside of breakable structure.",
                            ParserErrorType::InvalidContinuableScope => "not inside of continuable structure.",
                            ParserErrorType::UnexpectedToken(..) => "unexpected token",
                            ParserErrorType::ExpectedIdentifier => "expected identifier",
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

    #[cfg(debug_assertions)]
    dbg!(&ast);

    let code = Compiler::new(ast.into_iter()).build();

    #[cfg(debug_assertions)]
    dbg!(&code);

    let mut vm = vm::VM::new(code); 

    vm.add_function("dbg", |_: &mut vm::VM, args: &[Value]| {
        for arg in args {
            println!("{:?}", arg);
        }

        Value::Null
    });

    vm.run();
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

    (path.to_string(), std::fs::read_to_string(source).unwrap())
}

fn exit(code: i32) -> ! {
    std::process::exit(code)
}