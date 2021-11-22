use tonic_compiler::compile;
use rquickjs::{Runtime, Context, bind, Func, Value, Rest};

pub fn println(vs: Rest<Value>) -> () {
    for v in vs.into_inner().into_iter() {
        println!("{}", match true {
            _ if v.is_string() => v.into_string().unwrap().to_string().unwrap(),
            _ => unimplemented!(),
        });
    } 
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
    let context: rquickjs::Context = Context::full(&runtime).unwrap();

    #[cfg(debug_assertions)]
    println!("=== EVAL ===");
    
    context.with(|ctx: rquickjs::Ctx| {
        let glob = ctx.globals();

        glob.set("println", Func::from(println)).unwrap();

        ctx.eval::<(), _>(r#"
            println("Printing!", "testing!")
        "#).unwrap();

        // let res: () = ctx.eval(compiled).unwrap();
    })
}

fn file() -> String {
    std::env::args().nth(1).unwrap()
}

fn read(path: String) -> String {
    std::fs::read_to_string(path).unwrap()
}