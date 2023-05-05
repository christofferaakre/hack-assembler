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
    fn test_files() {
        const files: [&str; 4] = ["examples/add/Add.asm", "examples/max/MaxL.asm", "examples/rect/RectL.asm", "examples/pong/PongL.asm"];
        
        for file in files {
            eprintln!("Testing file {file}");
            let in_code = std::fs::read_to_string(file).expect("Failed to read .asm file");
            let instructions = decode_instructions(&in_code).expect("Failed to decode instructions");
            let codegen = generate_code(&instructions);
            let expected = std::fs::read_to_string(file.replace(".asm", ".correct.hack")).expect("Failed to read .correct.hack file");

            assert_eq!(codegen, expected);
        }
    }
}
