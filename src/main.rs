use std::{
    env, fs,
    io::{self},
};

use crate::compiler::compiler::compile;

mod compiler;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(args.get(1).expect("No argument provided")),
        _ => panic!("Too many args"),
    }
}

fn repl() {
    loop {
        print!("$> ");
        let mut input: String = String::new();
        let _ = io::stdin().read_line(&mut input);
        compile(input);
    }
}

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(input) => {
            compile(input);
        }
        Err(_) => panic!("File error"),
    }
}
