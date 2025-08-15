use crate::vm::{
    constant::Constant,
    instruction::{Binary, Comparison, Instruction},
    program::Program,
};

pub struct VirtualMachine {
    program: Program,
    pointer: usize,
    stack: Vec<Constant>,
}

impl VirtualMachine {
    pub fn new(program: Program) -> VirtualMachine {
        Self {
            program,
            pointer: 0,
            stack: Vec::new(),
        }
    }

    pub fn execute(&mut self) {
        self.program.add_instruction(Instruction::Return);
        while let Ok(()) = self.execute_instruction() {
            self.pointer += 1;
        }
    }

    fn pop_value(&mut self) -> Option<Constant> {
        let top = self.stack.pop();

        if let Some(Constant::Variable(identifer)) = top {
            self.program.get_variable(identifer)
        } else {
            top
        }
    }

    fn execute_instruction(&mut self) -> Result<(), VirtualMachineError> {
        match self.program.get_instruction(self.pointer).cloned() {
            Some(instruction) => match instruction {
                Instruction::Clear => {
                    self.stack.clear();
                    Ok(())
                }
                Instruction::Load(index) => match self.program.get_constant(index) {
                    Some(constant) => {
                        self.stack.push(constant.clone());
                        Ok(())
                    }

                    None => Err(VirtualMachineError::ExecutionError),
                },

                Instruction::Return => match self.pop_value() {
                    Some(value) => {
                        println!("{}", value);
                        Ok(())
                    }

                    None => Err(VirtualMachineError::ExecutionError),
                },

                Instruction::Not => {
                    if let Some(Constant::Boolean(boolean)) = self.pop_value() {
                        self.stack.push(Constant::Boolean(!boolean));
                        Ok(())
                    } else {
                        panic!("ERROR: Invalid operand")
                    }
                }

                Instruction::Negate => match self.pop_value() {
                    Some(value) => match value {
                        Constant::Integer(integer) => {
                            self.stack.push(Constant::Integer(-integer));
                            Ok(())
                        }

                        Constant::Float(float) => {
                            self.stack.push(Constant::Float(-float));
                            Ok(())
                        }

                        _ => panic!("ERROR: Invalid operand"),
                    },

                    None => Err(VirtualMachineError::ExecutionError),
                },

                Instruction::Reciprocate => match self.pop_value() {
                    Some(value) => match value {
                        Constant::Integer(integer) => {
                            self.stack.push(Constant::Float(1 as f32 / integer as f32));
                            Ok(())
                        }

                        Constant::Float(float) => {
                            self.stack.push(Constant::Float(1 as f32 / float));
                            Ok(())
                        }

                        _ => panic!("ERROR: Invalid operand"),
                    },

                    None => Err(VirtualMachineError::ExecutionError),
                },

                Instruction::Binary(operation) => match self.pop_value() {
                    Some(b) => match self.pop_value() {
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

                Instruction::Comparison(operation) => match self.pop_value() {
                    Some(b) => match self.pop_value() {
                        Some(a) => match operation {
                            Comparison::Equal => {
                                self.stack.push(a.equal(b));
                                Ok(())
                            }

                            Comparison::GreaterThan => match a.greater_than(b) {
                                Some(result) => {
                                    self.stack.push(result);
                                    Ok(())
                                }
                                None => panic!("ERROR"),
                            },

                            Comparison::GreaterThanEqual => match a.greater_than_equal(b) {
                                Some(result) => {
                                    self.stack.push(result);
                                    Ok(())
                                }
                                None => panic!("ERROR"),
                            },

                            Comparison::LessThan => match a.less_than(b) {
                                Some(result) => {
                                    self.stack.push(result);
                                    Ok(())
                                }
                                None => panic!("ERROR"),
                            },

                            Comparison::LessThanEqual => match a.less_than_equal(b) {
                                Some(result) => {
                                    self.stack.push(result);
                                    Ok(())
                                }
                                None => panic!("ERROR"),
                            },
                        },
                        None => panic!("compare fail"),
                    },
                    None => panic!("compare fail"),
                },
            },

            None => Err(VirtualMachineError::ExecutionError),
        }
    }
}

pub enum VirtualMachineError {
    CompilationError,
    ExecutionError,
}
