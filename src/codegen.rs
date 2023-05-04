use crate::instruction::Instruction;

pub fn generate_code(instructions: &[Instruction]) -> String {
    let binary_instructions_codegen: Vec<String> = instructions.iter().map(Instruction::codegen).collect();
    binary_instructions_codegen.join("\n")
}
