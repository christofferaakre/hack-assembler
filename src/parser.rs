use std::collections::HashSet;

use anyhow::Result;

use crate::{instruction::Instruction, jump::Jump, ops::Operation, register::Register};

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
                .skip(line.chars().position(|c| c == ';').unwrap())
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
    }
}
