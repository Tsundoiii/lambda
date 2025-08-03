use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Constant {
    Integer(i32),
}

impl Constant {
    pub fn add(&self, b: Constant) -> Option<Constant> {
        match self {
            Constant::Integer(a) => match b {
                Constant::Integer(b) => Some(Constant::Integer(a + b)),
            },
        }
    }

    pub fn multiply(&self, b: Constant) -> Option<Constant> {
        match self {
            Constant::Integer(a) => match b {
                Constant::Integer(b) => Some(Constant::Integer(a * b)),
            },
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Integer(integer) => write!(f, "{}", integer),
        }
    }
}
