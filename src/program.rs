use std::fmt::{Debug, Display};

use crate::types::Constant;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Load(usize),
    Return,
    Negate,
    Binary(Binary),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Binary {
    Add,
    Multiply,
}

pub struct Program {
    instructions: Vec<Instruction>,
    constants: Vec<Constant>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            instructions: Vec::new(),
            constants: vec![Constant::Integer(34), Constant::Integer(2)],
        }
    }

    pub fn add(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn get_instruction(&self, pointer: usize) -> Option<&Instruction> {
        self.instructions.get(pointer)
    }

    pub fn get_constant(&self, index: usize) -> Option<&Constant> {
        self.constants.get(index)
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
