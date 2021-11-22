use tonic_compiler::compile;
use rquickjs::{Runtime, Context, Func, Value, Rest};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "debug", short = "d", help = "Output debug information (JS, memory usage, etc)")]
    debug: bool,

    file: String,
}

pub fn println(vs: Rest<Value>) {
    for v in vs.into_inner().into_iter() {
        println!("{}", match true {
            _ if v.is_string() => v.into_string().unwrap().to_string().unwrap(),
            _ if v.is_number() => v.as_number().unwrap().to_string(),
            _ => unimplemented!(),
        });
    } 
}

fn main() {
    let args = Cli::from_args();

    let contents = read(args.file);
    let compiled = compile(&contents[..]);

    if args.debug {
        println!("=== JS OUTPUT ===");
        println!("{}", compiled);
    }

    let runtime: Runtime = Runtime::new().unwrap();
    runtime.set_max_stack_size(256 * 2048);

    let context: rquickjs::Context = Context::full(&runtime).unwrap();

    if args.debug {
        println!("=== EVAL ===");
    }
    
    context.with(|ctx: rquickjs::Ctx| {
        let glob = ctx.globals();

        glob.set("println", Func::from(println)).unwrap();

        ctx.eval::<(), _>(compiled).unwrap();
    });

    if args.debug {
        println!("=== DEBUG ===");
        println!("Memory used (bytes): {}", runtime.memory_usage().memory_used_size);
    }
}

fn read(path: String) -> String {
    std::fs::read_to_string(path).unwrap()
}