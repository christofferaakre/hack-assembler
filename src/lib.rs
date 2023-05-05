pub mod codegen;
pub mod instruction;
pub mod jump;
pub mod ops;
pub mod parser;
pub mod register;

pub type ValueType = u16;

#[cfg(test)]
mod tests {
    use crate::{codegen::generate_code, parser::decode_instructions};

    #[test]
    fn test_add() {
        let instructions = decode_instructions(include_str!("../examples/add/Add.asm")).unwrap();
        let code = generate_code(&instructions);

        let expected = include_str!("../examples/add/Add.correct.hack");

        println!("instructions: {instructions:?}");

        assert_eq!(code, expected);
    }

    #[test]
    fn test_max() {
        let instructions = decode_instructions(include_str!("../examples/max/MaxL.asm")).unwrap();
        let code = generate_code(&instructions);
        let expected = include_str!("../examples/max/MaxL.hack");
        println!("instructions: {instructions:?}");
        assert_eq!(code, expected);
    }
}
