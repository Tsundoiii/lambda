use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Clear,
    Load(usize),
    Return,
    Not,
    Negate,
    Reciprocate,
    Binary(Binary),
    Comparison(Comparison),
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

#[derive(Copy, Clone, Debug)]
pub enum Comparison {
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}
