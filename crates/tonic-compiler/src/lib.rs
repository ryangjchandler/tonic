use tonic_parser::{parse, Lexer, Statement};
pub use tonic_parser::{Token, TokenKind};

mod compiler;

pub fn compile(source: &str) -> (String, Vec<String>) {
    let ast = parse(source).unwrap();
    let mut imports = Vec::new();

    for import in ast.iter().filter(|n| matches!(n, Statement::Use { .. })) {
        imports.push(match import { Statement::Use { module, .. } => module.clone(), _ => unreachable!() });
    }

    let mut compiler = compiler::Compiler::new(ast.into_iter());

    (compiler.compile(), imports)
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        tokens.push(token);
    }

    tokens
}