use crate::{
    compiler::{scanner::Scanner, token::TokenType},
    vm::{
        constant::Constant,
        instruction::{Binary, Instruction},
        program::Program,
    },
};

pub fn compile(input: String) -> Option<Program> {
    let mut scanner = Scanner::new(input);
    let mut program: Program = Program::new();

    while let Ok(token) = scanner.scan_token() {
        dbg!(token);
        match token.token_type {
            TokenType::Integer(integer) => {
                let i = program.add_constant(Constant::Integer(integer));
                program.add_instruction(Instruction::Load(i));
            }

            TokenType::Float(float) => {
                let i = program.add_constant(Constant::Float(float));
                program.add_instruction(Instruction::Load(i));
            }

            TokenType::Plus => {
                program.add_instruction(Instruction::Binary(Binary::Add));
            }

            TokenType::Minus => {
                program.add_instruction(Instruction::Negate);
                program.add_instruction(Instruction::Binary(Binary::Add));
            }

            TokenType::Multiply => {
                program.add_instruction(Instruction::Binary(Binary::Multiply));
            }

            TokenType::Divide => {
                program.add_instruction(Instruction::Reciprocate);
                program.add_instruction(Instruction::Binary(Binary::Multiply));
            }

            _ => todo!(),
        }
    }

    Some(program)
}
