use crate::compiler::token::{Token, TokenType};

pub struct Scanner {
    input: String,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(input: String) -> Self {
        Self {
            input,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_token(&mut self) -> Result<Token, Token> {
        self.start = self.current;

        if self.start >= self.input.len() - 1 {
            return Err(Token::from_type(
                TokenType::End,
                self.current(),
                self.start,
                self.line,
            ));
        }

        self.skip_whitespace();

        self.current += 1;

        match self.current() {
            "<" | ">" => {
                self.current += 1;

                match Token::from_lexeme(self.current(), self.start, self.line) {
                    Some(token) => Ok(token),
                    None => {
                        self.current -= 1;

                        match Token::from_lexeme(self.current(), self.start, self.line) {
                            Some(token) => Ok(token),
                            None => {
                                todo!();
                            }
                        }
                    }
                }
            }
            _ => match Token::from_lexeme(self.current(), self.start, self.line) {
                Some(token) => Ok(token),

                None => match self.current() {
                    "\"" => {
                        while self.peek() != '"' {
                            self.current += 1;
                        }

                        self.current += 1;

                        Ok(Token::from_type(
                            TokenType::String,
                            self.current(),
                            self.start,
                            self.line,
                        ))
                    }

                    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                        while self.peek().is_digit(10) {
                            self.current += 1;
                        }

                        if self.peek() == '.' && self.peek_next().is_digit(10) {
                            self.current += 1;

                            while self.peek().is_digit(10) {
                                self.current += 1;
                            }
                        }

                        if self.current().contains(".") {
                            Ok(Token::from_type(
                                TokenType::Float(
                                    self.current()
                                        .parse::<f32>()
                                        .expect("could not parse float"),
                                ),
                                self.current(),
                                self.start,
                                self.line,
                            ))
                        } else {
                            Ok(Token::from_type(
                                TokenType::Integer(
                                    self.current().parse::<i32>().expect("could not parse int"),
                                ),
                                self.current(),
                                self.start,
                                self.line,
                            ))
                        }
                    }

                    _ => {
                        while self.peek().is_alphanumeric() {
                            self.current += 1;
                        }

                        Ok(Token::from_type(
                            TokenType::Identifier(self.current().to_string()),
                            self.current(),
                            self.start,
                            self.line,
                        ))
                    }
                },
            },
        }
    }

    fn current(&self) -> &str {
        &self.input[self.start..self.current]
    }

    fn peek(&self) -> char {
        *self.input[self.current..self.current + 1]
            .chars()
            .collect::<Vec<char>>()
            .get(0)
            .expect("Char not present")
    }

    fn peek_next(&self) -> char {
        *self.input[self.current + 1..self.current + 2]
            .chars()
            .collect::<Vec<char>>()
            .get(0)
            .expect("Char not present")
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            if &self.input[self.start..self.current] == "\n" {
                self.line += 1;
            }

            self.current += 1;
            self.start += 1;
        }
    }
}
