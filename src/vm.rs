use crate::{
    program::{Binary, Instruction, Program},
    types::Constant,
};

pub struct VirtualMachine {
    program: Program,
    pub pointer: usize,
    stack: Vec<Constant>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            program: Program::new(),
            pointer: 0,
            stack: Vec::new(),
        }
    }

    pub fn from_program(program: Program) -> VirtualMachine {
        VirtualMachine {
            program,
            pointer: 0,
            stack: Vec::new(),
        }
    }

    pub fn execute_instruction(&mut self) -> Result<(), VirtualMachineError> {
        match self.program.get_instruction(self.pointer) {
            Some(instruction) => match instruction {
                Instruction::Load(index) => match self.program.get_constant(*index) {
                    Some(constant) => {
                        self.stack.push(*constant);
                        Ok(())
                    }

                    None => Err(VirtualMachineError::ExecutionError),
                },
                Instruction::Return => match self.stack.pop() {
                    Some(value) => {
                        println!("{}", value);
                        Ok(())
                    }

                    None => Err(VirtualMachineError::ExecutionError),
                },
                Instruction::Negate => match self.stack.pop() {
                    Some(value) => match value {
                        Constant::Integer(integer) => {
                            self.stack.push(Constant::Integer(-integer));
                            Ok(())
                        }
                    },

                    None => Err(VirtualMachineError::ExecutionError),
                },

                Instruction::Binary(operation) => match self.stack.pop() {
                    Some(b) => match self.stack.pop() {
                        Some(a) => match operation {
                            Binary::Add => match a.add(b) {
                                Some(result) => {
                                    self.stack.push(result);
                                    Ok(())
                                }

                                None => Err(VirtualMachineError::ExecutionError),
                            },

                            Binary::Multiply => match a.multiply(b) {
                                Some(result) => {
                                    self.stack.push(result);
                                    Ok(())
                                }

                                None => Err(VirtualMachineError::ExecutionError),
                            },
                        },

                        None => Err(VirtualMachineError::ExecutionError),
                    },

                    None => Err(VirtualMachineError::ExecutionError),
                },
            },

            None => Ok(()),
        }
    }
}

pub enum VirtualMachineError {
    CompilationError,
    ExecutionError,
}
