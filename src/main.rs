use crate::program::{Instruction, Program};

mod program;
mod types;

fn main() {
    let mut p = Program::new();
    p.add(Instruction::Return);
    p.add(Instruction::Load(0));
    dbg!(p);
}
