use tonic_compiler::compile;

fn main() {
    let file = file();
    let contents = read(file);
    let compiled = compile(&contents[..]);

    println!("=== JS OUTPUT ===");
    println!("{}", compiled);
}

fn file() -> String {
    std::env::args().nth(1).unwrap()
}

fn read(path: String) -> String {
    std::fs::read_to_string(path).unwrap()
}