use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Constant {
    Boolean(bool),
    Integer(i32),
    Float(f32),
}

impl Constant {
    pub fn add(&self, b: Constant) -> Option<Self> {
        match self {
            Self::Integer(a) => match b {
                Self::Integer(b) => Some(Self::Integer(a + b)),
                Self::Float(b) => Some(Self::Float(*a as f32 + b)),
                _ => panic!("ERROR: Invalid operand"),
            },

            Self::Float(a) => match b {
                Self::Integer(b) => Some(Self::Float(a + b as f32)),
                Self::Float(b) => Some(Self::Float(a + b)),
                _ => panic!("ERROR: Invalid operand"),
            },

            _ => panic!("ERROR: Invalid operand"),
        }
    }

    pub fn multiply(&self, b: Constant) -> Option<Self> {
        match self {
            Self::Integer(a) => match b {
                Self::Integer(b) => Some(Self::Integer(a * b)),
                Self::Float(b) => Some(Self::Float(*a as f32 * b)),
                _ => panic!("ERROR: Invalid operand"),
            },

            Self::Float(a) => match b {
                Self::Integer(b) => Some(Self::Float(a * b as f32)),
                Self::Float(b) => Some(Self::Float(a * b)),
                _ => panic!("ERROR: Invalid operand"),
            },

            _ => panic!("ERROR: Invalid operand"),
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::Float(float) => write!(f, "{}", float),
        }
    }
}
