use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Load(usize),
    Return,
    Negate,
    Reciprocate,
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
