use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Constant {
    Boolean(bool),
    Integer(i32),
    Float(f32),
}

impl Constant {
    pub fn add(&self, b: Self) -> Option<Self> {
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

    pub fn multiply(&self, b: Self) -> Option<Self> {
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

    pub fn equal(&self, b: Self) -> Self {
        match self {
            Self::Boolean(a) => match b {
                Self::Boolean(b) => Self::Boolean(*a == b),
                _ => Self::Boolean(false),
            },

            Self::Integer(a) => match b {
                Self::Integer(b) => Self::Boolean(*a == b),
                _ => Self::Boolean(false),
            },

            Self::Float(a) => match b {
                Self::Float(b) => Self::Boolean(*a == b),
                _ => Self::Boolean(false),
            },
        }
    }

    pub fn greater_than(&self, b: Self) -> Option<Self> {
        match self {
            Self::Integer(a) => match b {
                Self::Integer(b) => Some(Self::Boolean(*a > b)),
                Self::Float(b) => Some(Self::Boolean(*a as f32 > b)),
                _ => None,
            },

            Self::Float(a) => match b {
                Self::Integer(b) => Some(Self::Boolean(*a > b as f32)),
                Self::Float(b) => Some(Self::Boolean(*a > b)),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn greater_than_equal(&self, b: Self) -> Option<Self> {
        match self {
            Self::Integer(a) => match b {
                Self::Integer(b) => Some(Self::Boolean(*a >= b)),
                Self::Float(b) => Some(Self::Boolean(*a as f32 >= b)),
                _ => None,
            },

            Self::Float(a) => match b {
                Self::Integer(b) => Some(Self::Boolean(*a >= b as f32)),
                Self::Float(b) => Some(Self::Boolean(*a >= b)),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn less_than(&self, b: Self) -> Option<Self> {
        match self {
            Self::Integer(a) => match b {
                Self::Integer(b) => Some(Self::Boolean(*a < b)),
                Self::Float(b) => Some(Self::Boolean((*a as f32) < b)),
                _ => None,
            },

            Self::Float(a) => match b {
                Self::Integer(b) => Some(Self::Boolean(*a < b as f32)),
                Self::Float(b) => Some(Self::Boolean(*a < b)),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn less_than_equal(&self, b: Self) -> Option<Self> {
        match self {
            Self::Integer(a) => match b {
                Self::Integer(b) => Some(Self::Boolean(*a <= b)),
                Self::Float(b) => Some(Self::Boolean((*a as f32) <= b)),
                _ => None,
            },

            Self::Float(a) => match b {
                Self::Integer(b) => Some(Self::Boolean(*a <= b as f32)),
                Self::Float(b) => Some(Self::Boolean(*a <= b)),
                _ => None,
            },
            _ => None,
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
