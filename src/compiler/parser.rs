use crate::{
    compiler::{
        scanner::Scanner,
        token::{Token, TokenType},
    },
    vm::{constant::Constant, instruction::Instruction, program::Program},
};

pub struct Parser {
    scanner: Scanner,
    program: Program,
    previous: Token,
    current: Token,
    had_error: bool,
}

impl Parser {
    pub fn advance(&mut self) {
        self.previous = self.current;

        while let Ok(token) = self.scanner.scan_token() {
            self.current = token;
        }
    }

    pub fn consume(&mut self, token_type: TokenType, message: String) {
        if (self.current.token_type == token_type) {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn error_at(&mut self, token: Token, message: String) {
        if self.had_error {
            return;
        }

        self.had_error = true;

        eprintln!(
            "[line {} char {}] Error: {}",
            token.line, token.start, message
        )
    }

    fn error_at_current(&mut self, message: String) {
        self.error_at(self.current, message);
    }

    fn error_at_previous(&mut self, message: String) {
        self.error_at(self.previous, message);
    }

    fn expression(&mut self) {
        todo!()
    }

    fn integer(&mut self) {
        if let TokenType::Integer(integer) = self.previous.token_type {
            let index: usize = self.program.add_constant(Constant::Integer(integer));
            self.program.add_instruction(Instruction::Load(index));
        }
    }

    fn unary(&mut self) {
        let token_type = self.previous.token_type;

        self.expression();

        match token_type {
            TokenType::Minus => self.program.add_instruction(Instruction::Negate),
            _ => todo!(),
        }
    }

    fn binary(&mut self) {}
}
