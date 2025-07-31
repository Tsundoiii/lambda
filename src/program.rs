use std::fmt::{Debug, Display};

use crate::types::Constant;

#[derive(Clone, Debug)]
pub enum Instruction {
    Return,
    Load(usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Program {
    instructions: Vec<Instruction>,
    constants: Vec<Constant>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            instructions: Vec::new(),
            constants: vec![Constant::Integer(34)],
        }
    }

    pub fn add(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
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
