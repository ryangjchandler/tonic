use tonic_compiler::compile;
use rquickjs::{Runtime, Context, bind};

#[bind(object)]
pub fn println(s: String) -> () {
    println!("{}", s);
}

fn main() {
    let file = file();
    let contents = read(file);
    let compiled = compile(&contents[..]);

    #[cfg(debug_assertions)]
    {
        println!("=== JS OUTPUT ===");
        println!("{}", compiled);
    }

    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();

    #[cfg(debug_assertions)]
    println!("=== EVAL ===");
    
    context.with(|ctx| {
        let glob = ctx.globals();
        glob.init_def::<Println>().unwrap();

        let res: () = ctx.eval(compiled).unwrap();
    })
}

fn file() -> String {
    std::env::args().nth(1).unwrap()
}

fn read(path: String) -> String {
    std::fs::read_to_string(path).unwrap()
}