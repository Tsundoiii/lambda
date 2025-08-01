use crate::{
    program::{Binary, Instruction, Program},
    vm::VirtualMachine,
};

mod program;
mod types;
mod vm;

fn main() {
    let mut p = Program::new();
    p.add(Instruction::Load(0));
    p.add(Instruction::Load(1));
    p.add(Instruction::Negate);
    p.add(Instruction::Binary(Binary::Multiply));
    p.add(Instruction::Return);
    let mut vm = VirtualMachine::from_program(p);
    while let Ok(()) = vm.execute_instruction() {
        vm.pointer += 1;
    }
}
