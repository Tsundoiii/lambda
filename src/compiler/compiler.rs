use crate::{compiler::scanner::Scanner, vm::program::Program};

pub fn compile(input: String) -> Option<Program> {
    let mut scanner = Scanner::new(input);
    while let Ok(token) = scanner.scan_token() {
        dbg!(token);
    }
    todo!();
}
