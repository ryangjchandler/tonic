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

const POLYFILL: &str = include_str!("../js/polyfill.js");
const WEB_MODULE: &str = include_str!("../dist/web.js");
const JSON_MODULE: &str = include_str!("../js/json.js");

pub fn println(vs: Rest<Value>) {
    fn stringify(v: Value) -> String {
        match true {
            _ if v.is_string() => v.into_string().unwrap().to_string().unwrap(),
            _ if v.is_number() => v.as_number().unwrap().to_string(),
            _ if v.is_bool() => v.as_bool().unwrap().to_string(),
            _ if v.is_array() => v.into_array().unwrap().into_iter().map(|v| stringify(v.unwrap())).collect::<Vec<String>>().join(", "),
            _ => unimplemented!(),
        }
    }

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

        pub fn contents(&self) -> String {
            self.contents.clone()
        }

        pub fn read(path: String) -> Self {
            Self::new(path)
        }
    }
}

#[bind(module, public)]
#[quickjs(bare)]
mod env {
    use std::env::{var};

    pub fn get(name: String) -> String {
        match var(name) {
            Ok(value) => value,
            Err(_) => unreachable!()
        }
    }

    pub fn has(name: String) -> bool {
        var(name).is_ok()
    }
}

#[bind(module, public)]
#[quickjs(bare)]
mod uuid {
    use uuid::Uuid as UuidGenerator;

    #[derive(Clone)]
    #[quickjs(cloneable)]
    pub struct Uuid {
        value: String
    }

    impl Uuid {
        pub fn new() -> Self {
            Self {
                value: UuidGenerator::new_v4().to_string()
            }
        }

        pub fn to_string(&self) -> String {
            self.value.clone()
        }

        pub fn generate() -> String {
            UuidGenerator::new_v4().to_string()
        }
    }
}

#[bind(module, public)]
#[quickjs(bare)]
mod http {
    use tiny_http::{Server as TinyHttpServer, Response};
    use rquickjs::{Value, Function};

    #[derive(Clone)]
    #[quickjs(cloneable)]
    pub struct Server {
        target: String
    }

    impl Server {
        pub fn new(target: String) -> Self {
            Self {
                target,
            }
        }

        pub fn serve(&self, callback: Value) {
            let callback: Function = callback.into_function().unwrap();
            let server = TinyHttpServer::http(self.target.clone()).unwrap();

            for request in server.incoming_requests() {
                let method = request.method();
                let url = request.url();

                let response = Response::from_string(callback.call::<_, String>((method.as_str(), url)).unwrap());
                request.respond(response).unwrap();
            }
        }

        pub fn init(target: String) -> Self {
            Self::new(target)
        }
    }
}

fn main() {
    let args = Cli::from_args();

    let runtime: Runtime = Runtime::new().unwrap();
    runtime.set_max_stack_size(256 * 2048);

    let resolver = (
        BuiltinResolver::default()
            .with_module("@std/fs")
            .with_module("@std/env")
            .with_module("@std/uuid")
            .with_module("@std/http")
            .with_module("@std/web")
            .with_module("@std/json"),
        FileResolver::default()
            .with_path("./"),
    );

    let loader = (
        BuiltinLoader::default()
            .with_module("@std/web", WEB_MODULE)
            .with_module("@std/json", JSON_MODULE),
        ModuleLoader::default()
            .with_module("@std/fs", Fs)
            .with_module("@std/env", Env)
            .with_module("@std/uuid", Uuid)
            .with_module("@std/http", Http),
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
        println!("Tonic REPL v0.3.0");
        
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