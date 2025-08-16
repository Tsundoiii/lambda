use std::{
    env, fs,
    io::{self, Write},
};

use crate::{compiler::compiler::compile, vm::vm::VirtualMachine};

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
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut input);
        input = input.trim().to_string();
        input.push_str(" print\n");
        execute(input);
    }
}

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(input) => execute(input),

        Err(_) => panic!("File error"),
    }
}

fn execute(input: String) {
    if let Some(program) = compile(input) {
        VirtualMachine::new(program).execute();
    }
}
