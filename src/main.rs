use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 => Prompt::run(),
        1 => File::run(),
        _ => println!("Too many arguments. Usage: lox [script]")
    }
}

struct Prompt {}

impl Prompt {
    fn new() -> Self {
        todo!()
    }

    fn run() {
        todo!()
    }
}


struct File {}

impl File {
    fn new() -> Self {
        todo!()
    }

    fn run() {
        todo!()
    }
}
