pub mod register;
pub mod ops;
pub mod jump;
pub mod instruction;
pub mod parser;
pub mod codegen;

pub type ValueType = u16;

#[cfg(test)]
mod tests {
    use crate::{parser::decode_instructions, codegen::generate_code};

    #[test]
    fn test_add() {
        let instructions = decode_instructions(include_str!("../examples/add/Add.asm")).unwrap();
        let code = generate_code(&instructions);

        let expected = include_str!("../examples/add/Add.correct.hack");

        println!("instructions: {instructions:?}");

        assert_eq!(code, expected);
    }
}
