use std::fmt::Display;

pub enum Constant {
    Integer(i32),
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Integer(integer) => write!(f, "{}", integer),
        }
    }
}
