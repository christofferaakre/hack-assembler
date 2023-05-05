use crate::instruction::Instruction;

pub fn generate_code(instructions: &[Instruction]) -> String {
    let binary_instructions_codegen: Vec<String> =
        instructions.iter().map(Instruction::codegen).collect();
    let mut s = binary_instructions_codegen.join("\n");
    s.push('\n');
    s
}
