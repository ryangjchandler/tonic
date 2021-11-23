use tonic_compiler::compile;
use rquickjs::{BuiltinLoader, BuiltinResolver, FileResolver, Runtime, ModuleLoader, ScriptLoader, Context, Func, Value, Rest, bind, qjs::JSValue};
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

const POLYFILL: &str = include_str!("polyfill.js");

pub fn println(vs: Rest<Value>) {
    fn stringify(v: Value) -> String {
        match true {
            _ if v.is_string() => v.into_string().unwrap().to_string().unwrap(),
            _ if v.is_number() => v.as_number().unwrap().to_string(),
            _ if v.is_bool() => v.as_bool().unwrap().to_string(),
            _ if v.is_array() => v.into_array().unwrap().into_iter().map(|v| stringify(v.unwrap())).collect::<Vec<String>>().join(", "),
            _ => unimplemented!(),
        }
    };

    for v in vs.into_inner().into_iter() {
        println!("{}", stringify(v));
    }
}

#[bind(module, public)]
#[quickjs(bare)]
mod fs {
    #[derive(Clone)]
    #[quickjs(cloneable)]
    pub struct File {
        path: String,
        contents: String,
    }

    impl File {
        pub fn new(path: String) -> Self {
            // TODO: Check the file exists before trying to read it.
            Self {
                path: path.clone(),
                contents: std::fs::read_to_string(path).unwrap(),
            }
        }

        pub fn path(&self) -> String {
            self.path.clone()
        }

        pub fn lines(&self) -> Vec<&str> {
            self.contents.lines().collect()
        }

        pub fn is_empty(&self) -> bool {
            self.contents.is_empty()
        }

        pub fn exists(path: String) -> bool {
            std::fs::metadata(path).is_ok()
        }

        pub fn read(path: String) -> Self {
            Self::new(path)
        }
    }
}

fn main() {
    let args = Cli::from_args();
    let runtime: Runtime = Runtime::new().unwrap();

    runtime.set_max_stack_size(256 * 2048);

    let resolver = (
        BuiltinResolver::default()
            .with_module("@std/fs"),
        FileResolver::default()
            .with_path("./"),
    );

    let loader = (
        BuiltinLoader::default(),
        ModuleLoader::default()
            .with_module("@std/fs", Fs),
        ScriptLoader::default(),
    );

    runtime.set_loader(resolver, loader);

    let context: rquickjs::Context = Context::full(&runtime).unwrap();
    
    if let Some(file) = args.file {
        let contents = read(file.clone());
        let compiled = [
            POLYFILL.to_string(),
            if args.raw { contents } else { compile(&contents[..]) }
        ].join("\n");

        let fqp = std::fs::canonicalize(file.clone()).unwrap();
        let fqd = fqp.parent().unwrap();
    
        if args.debug {
            println!("=== JS OUTPUT ===");
            println!("{}", compiled);
        }

        context.with(|ctx: rquickjs::Ctx| {
            let glob = ctx.globals();
    
            glob.set("println", Func::from(println)).unwrap();
            glob.set("__FILE__", fqp.to_str()).unwrap();
            glob.set("__DIR__", fqd.to_str()).unwrap();
    
            if args.debug {
                println!("=== EVAL ===");
            }
            
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