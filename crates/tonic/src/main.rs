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
mod http {
    use std::collections::HashMap;
    use ureq::{get, post, Request};

    #[derive(Clone)]
    pub enum ClientMethod {
        Get,
        Post,
    }

    #[derive(Clone)]
    #[quickjs(cloneable)]
    pub struct Client {
        method: ClientMethod,
        path: String,
        headers: HashMap<String, String>,
    }

    impl Client {
        pub fn new() -> Self {
            Self {
                method: ClientMethod::Get,
                path: String::default(),
                headers: HashMap::default(),
            }
        }

        pub fn get(&mut self, path: String) -> &mut Self {
            self.path = path;
            self.method = ClientMethod::Get;
            self
        }

        pub fn header(&mut self, name: String, value: String) -> &mut Self {
            self.headers.insert(name, value);
            self
        }

        pub fn send(&self) -> String {
            let mut request: Request = match self.method {
                ClientMethod::Get => get(&self.path),
                ClientMethod::Post => post(&self.path),
            };

            for (header, value) in self.headers.clone() {
                request = request.set(&header, &value);
            }

            request.call().unwrap().into_string().unwrap()
        }
    }
}

fn main() {
    let args = Cli::from_args();
    let runtime: Runtime = Runtime::new().unwrap();

    runtime.set_max_stack_size(256 * 2048);

    let resolver = (
        BuiltinResolver::default()
            .with_module("@std/http"),
        FileResolver::default()
            .with_path("./"),
    );

    let loader = (
        BuiltinLoader::default(),
        ModuleLoader::default()
            .with_module("@std/http", Http),
        ScriptLoader::default(),
    );

    runtime.set_loader(resolver, loader);

    let context: rquickjs::Context = Context::full(&runtime).unwrap();
    
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