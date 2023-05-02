use std::collections::HashSet;

use anyhow::Result;

use crate::{instruction::Instruction, jump::Jump, ops::Operation, register::Register};

pub fn decode_instructions(code: &str) -> Result<Vec<Instruction>> {
    let lines = code.lines();
    let instructions = lines
        .map(clean_line)
        .filter_map(|l| l)
        .map(|l| decode_instruction(l.as_str()));
    instructions.collect()
}

pub fn decode_instruction(line: &str) -> Result<Instruction> {
    // Handle comments
    if line.starts_with("//") {
        return Ok(Instruction::Comment);
    }

    // Handle A instruction: @xxx
    if line.starts_with('@') {
        // skip the @
        let value: u16 = line[1..].parse()?;
        return Ok(Instruction::A { value });
    }
    // Handle C instruction:
    // D=A+1
    //D=D+A
    //0;JNE
    //AM=M-1

    // handle destinations before "="
    let dest: HashSet<_> = if line.contains('=') {
        line.chars()
            .take_while(|c| *c != '=')
            .map(|c| Register::try_from(c))
            .map(|r| r)
            .collect()
    } else {
        Ok(HashSet::new())
    }?;

    // skip until after "="
    // then handle all chars until ";" if ; exists, otherwise until end
    let op_str: String = line
        .chars()
        .skip(line.chars().position(|c| c == '=').unwrap_or(0) + 1)
        .take_while(|c| *c != ';')
        .collect();
    let operation = Operation::try_from(op_str)?;

    // skip until after ; if one exists
    // then handle all chars until end of line
    let jump_str: Option<String> = if line.contains(';') {
        Some(
            line.chars()
                .skip(line.chars().position(|c| c == ';').unwrap() + 1)
                .collect(),
        )
    } else {
        None
    };

    let jump = Jump::try_from(jump_str)?;

    Ok(Instruction::C {
        dest,
        comp: operation,
        jump,
    })
}

pub fn clean_line(line: &str) -> Option<String> {
    // return None if empty line
    if line.is_empty() {
        return None;
    }

    // remove all whitespace
    let cleaned_line: String = line.split_whitespace().collect();

    if cleaned_line.is_empty() {
        None
    } else {
        Some(cleaned_line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clean_line_works() {
        assert_eq!(clean_line(""), None);
        assert_eq!(clean_line("\n"), None);
        assert_eq!(
            clean_line("  1 2  3  4  5 6 7  "),
            Some("1234567".to_owned())
        );
    }

    #[test]
    fn decode_instruction_A() {
        let line = "@123";
        let r = decode_instruction(line);
        assert!(r.is_ok() && r.unwrap() == Instruction::A { value: 123 });

        let line = "@xyz";
        let r = decode_instruction(line);
        assert!(r.is_err());
    }

    #[test]
    fn decode_instruction_C() {
        let line = "AM=D+A;JEQ";
        let r = decode_instruction(line);

        assert!(
            r.is_ok()
                && dbg!(r.unwrap())
                    == Instruction::C {
                        dest: HashSet::from_iter([Register::A, Register::M]),
                        comp: Operation::DPlusA,
                        jump: Jump::Equal
                    }
        )
    }

    fn decode_instructions_add_asm() {
        let s = include_str!("../examples/add/Add.asm");
        let instructions = decode_instructions(s);
        assert!(
            instructions.is_ok()
                && instructions.unwrap()
                    == vec![
                        Instruction::A { value: 2 },
                        Instruction::C {
                            dest: HashSet::from_iter([Register::D]),
                            comp: Operation::A,
                            jump: Jump::NoJump
                        },
                        Instruction::A { value: 3 },
                        Instruction::C {
                            dest: HashSet::from_iter([Register::D]),
                            comp: Operation::DPlusA,
                            jump: Jump::NoJump
                        },
                        Instruction::A { value: 0 },
                        Instruction::C {
                            dest: HashSet::from_iter([Register::M]),
                            comp: Operation::D,
                            jump: Jump::NoJump
                        },
                    ]
        );
    }

    fn decode_instructions_bad_code() {
        let s = "asdfasdf\nasdfasdf\nasdf";
        let instructions = decode_instructions(s);
        assert!(instructions.is_err());
    }
}
