#![allow(dead_code)]
use anyhow::Result;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

mod token;
use token::Token;
mod lexer;
use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => Prompt::run().unwrap(),
        2 => {
            let file = File::new(&args[1]).expect("Failed to read file to string");
            file.run()
        }
        _ => println!("Too many arguments. Usage: lox [script]"),
    }
}

struct Prompt {}

impl Prompt {
    fn new() -> Self {
        todo!()
    }

    fn run() -> io::Result<()> {
        let stdin = io::stdin();
        print!("> ",);
        io::stdout().flush().unwrap();
        for line in stdin.lock().lines() {
            io::stdout().flush().unwrap();
            let lexer = Lexer::new(line.unwrap());
            let tokens = lexer.lex_tokens();
            println!("Parsed tokens: {:?}", tokens);
            print!("> ",);
            io::stdout().flush().unwrap();
        }
        Ok(())
    }
}

struct File {
    contents: String,
}

impl File {
    fn new(file_name: &String) -> Result<Self> {
        let contents = fs::read_to_string(file_name)?;
        Ok(File { contents })
    }

    fn run(self) {
        let lexer = Lexer::new(self.contents);
        let tokens = lexer.lex_tokens();
        println!("Parsed tokens: {:?}", tokens);
    }
}

struct LoxError;

impl LoxError {
    fn print(line: u32, at: &str, message: &str) {
        eprintln!("[Line {line}] Error {at}: {message}");
    }
}
