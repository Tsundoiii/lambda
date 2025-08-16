use crate::{
    compiler::{scanner::Scanner, token::TokenType},
    vm::{
        constant::Constant,
        instruction::{Binary, Comparison, Instruction},
        program::Program,
    },
};

pub fn compile(input: String) -> Option<Program> {
    let mut scanner = Scanner::new(input);
    let mut program: Program = Program::new();

    while let Ok(token) = scanner.scan_token() {
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

            TokenType::Equal => {
                program.add_instruction(Instruction::Comparison(Comparison::Equal));
            }

            TokenType::GreaterThan => {
                program.add_instruction(Instruction::Comparison(Comparison::GreaterThan));
            }

            TokenType::GreaterThanEqual => {
                program.add_instruction(Instruction::Comparison(Comparison::GreaterThanEqual));
            }

            TokenType::LessThan => {
                program.add_instruction(Instruction::Comparison(Comparison::LessThan));
            }

            TokenType::LessThanEqual => {
                program.add_instruction(Instruction::Comparison(Comparison::LessThanEqual));
            }

            TokenType::Semicolon => {
                program.add_instruction(Instruction::Clear);
            }

            TokenType::Identifier(identifier) => match identifier.as_str() {
                "bind" => {
                    program.add_variable();
                }

                "print" => {
                    program.add_instruction(Instruction::Return);
                }

                _ => {
                    let i = program.add_constant(Constant::Variable(identifier));
                    program.add_instruction(Instruction::Load(i));
                }
            },
            _ => todo!(),
        }
    }

    Some(program)
}
