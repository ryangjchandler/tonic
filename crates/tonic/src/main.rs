use tonic_compiler::compile;
use rquickjs::{BuiltinLoader, BuiltinResolver, FileResolver, Runtime, ModuleLoader, ScriptLoader, Context, Func, Value, Rest, bind};
use rustyline::{Editor, error::ReadlineError};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "debug", short = "d", help = "Output debug information (JS, memory usage, etc)")]
    debug: bool,

    #[structopt(long = "raw", short = "r", help = "Execute the specified file as raw JavaScript")]
    raw: bool,

    file: Option<String>,
}

pub fn println(vs: Rest<Value>) {
    for v in vs.into_inner().into_iter() {
        println!("{}", match true {
            _ if v.is_string() => v.into_string().unwrap().to_string().unwrap(),
            _ if v.is_number() => v.as_number().unwrap().to_string(),
            _ if v.is_bool() => v.as_bool().unwrap().to_string(),
            _ => unimplemented!(),
        });
    }
}

#[bind(module, public)]
#[quickjs(bare)]
mod testing {
    pub fn example() -> String {
        String::from("Example!")
    }
}

fn main() {
    let args = Cli::from_args();
    let runtime: Runtime = Runtime::new().unwrap();

    runtime.set_max_stack_size(256 * 2048);

    let resolver = (
        BuiltinResolver::default()
            .with_module("@std/testing"),
        FileResolver::default()
            .with_path("./"),
    );

    let loader = (
        BuiltinLoader::default(),
        ModuleLoader::default()
            .with_module("@std/testing", Testing),
        ScriptLoader::default(),
    );

    runtime.set_loader(resolver, loader);

    let context: rquickjs::Context = Context::full(&runtime).unwrap();

    if args.debug {
        println!("=== EVAL ===");
    }
    
    if let Some(file) = args.file {
        let contents = read(file.clone());
        let compiled = if args.raw { contents } else { compile(&contents[..]) };
    
        if args.debug {
            println!("=== JS OUTPUT ===");
            println!("{}", compiled);
        }

        context.with(|ctx: rquickjs::Ctx| {
            let glob = ctx.globals();
    
            glob.set("println", Func::from(println)).unwrap();
    
            ctx.compile(file, compiled).unwrap();
        });
    
        if args.debug {
            println!("=== DEBUG ===");
            println!("Memory used (bytes): {}", runtime.memory_usage().memory_used_size);
        }
    } else {
        println!("Tonic REPL v0.1.0");
        
        let mut rl = Editor::<()>::new();

        loop {
            let line = rl.readline(">> ");

            match line {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());

                    context.with(|ctx: rquickjs::Ctx| {
                        let glob = ctx.globals();
                
                        glob.set("println", Func::from(println)).unwrap();
                
                        ctx.eval::<(), _>(line).unwrap();
                    });
                },
                Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                    println!("Exiting!");
                    break
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    break
                }
            }
        }
    }
}

fn read(path: String) -> String {
    std::fs::read_to_string(path).unwrap()
}