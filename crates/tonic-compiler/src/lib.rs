use tonic_parser::{parse, Lexer};
pub use tonic_parser::{Token, TokenKind};

mod compiler;

pub fn compile(source: &str) -> String {
    let ast = parse(source).unwrap();

    let mut compiler = compiler::Compiler::new(ast.into_iter());
    compiler.compile()
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        tokens.push(token);
    }

    tokens
}