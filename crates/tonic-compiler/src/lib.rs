use tonic_parser::parse;

mod compiler;

pub fn compile(source: &str) -> String {
    let ast = parse(source).unwrap();

    let mut compiler = compiler::Compiler::new(ast.into_iter());
    compiler.compile()
}