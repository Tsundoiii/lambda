#[derive(Copy, Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: u32,
    pub line: u32,
}

impl Token {
    pub fn from_lexeme(lexeme: &str, start: usize, line: u32) -> Option<Self> {
        match TokenType::from(lexeme) {
            Some(token_type) => Some(Self {
                token_type,
                start,
                length: lexeme.len() as u32,
                line: line as u32,
            }),
            None => None,
        }
    }

    pub fn from_type(token_type: TokenType, string: &str, start: usize, line: u32) -> Self {
        Self {
            token_type,
            start,
            length: string.len() as u32,
            line: line as u32,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    String,
    Integer(i32),
    Float(f32),
    Identifier,

    Dot,

    Plus,
    Minus,
    Multiply,
    Divide,

    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,

    Hash,

    Error,
    End,
}

impl TokenType {
    pub fn from(lexeme: &str) -> Option<Self> {
        match lexeme {
            "." => Some(Self::Dot),
            "+" => Some(Self::Plus),
            "-" => Some(Self::Minus),
            "*" => Some(Self::Multiply),
            "/" => Some(Self::Divide),
            "=" => Some(Self::Equal),
            ">" => Some(Self::GreaterThan),
            ">=" => Some(Self::GreaterThanEqual),
            "<" => Some(Self::LessThan),
            "<=" => Some(Self::LessThanEqual),
            "#" => Some(Self::Hash),
            _ => None,
        }
    }
}
