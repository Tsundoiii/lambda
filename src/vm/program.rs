use std::{collections::HashMap, fmt::Debug};

use crate::vm::{constant::Constant, instruction::Instruction};

pub struct Program {
    instructions: Vec<Instruction>,
    constants: Vec<Constant>,
    variables: HashMap<String, Constant>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            instructions: Vec::new(),
            constants: Vec::new(),
            variables: HashMap::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn get_instruction(&self, pointer: usize) -> Option<&Instruction> {
        self.instructions.get(pointer)
    }

    pub fn add_constant(&mut self, constant: Constant) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> Option<&Constant> {
        self.constants.get(index)
    }

    pub fn add_variable(&mut self) {
        if let Some(Constant::Variable(identifer)) = self.constants.pop() {
            let value = self.constants.pop().expect("ERROR");
            self.variables.insert(identifer, value);
        }
    }

    pub fn get_variable(&self, identifer: String) -> Option<Constant> {
        self.variables.get(&identifer).cloned()
    }
}

impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            &self
                .instructions
                .iter()
                .enumerate()
                .map(|(count, instruction)| {
                    match instruction {
                        Instruction::Load(index) => format!(
                            "{} {} {}",
                            count,
                            instruction,
                            &self.constants.get(*index).unwrap()
                        ),
                        _ => format!("{} {}", count, instruction),
                    }
                })
                .collect::<Vec<String>>()
        )
    }
}
