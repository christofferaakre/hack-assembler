use clap::Parser as ClapParser;
use hack_assembler::{
    instruction::Instruction,
    parser::{clean_line, decode_instruction},
};
use std::{path::PathBuf, process::ExitCode};

use anyhow::Result;

#[derive(ClapParser, Debug)]
struct Args {
    program: PathBuf,
}

fn main() -> Result<ExitCode> {
    let args = Args::parse();

    let file_contents = std::fs::read_to_string(args.program)?;
    let lines = file_contents.lines();

    let mut line_number = 0;
    for line in lines {
        let line = clean_line(line);
        if let Some(line) = line {
            let statement = decode_instruction(&line)?;
            if statement != Instruction::Comment {
                line_number += 1;
            }

            println!("Line {line_number}, instruction {statement:?}");
        }
    }

    Ok(ExitCode::from(0))
}
