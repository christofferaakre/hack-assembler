use clap::Parser as ClapParser;
use hack_assembler::{
    instruction::Instruction,
    parser::{clean_line, decode_instruction, decode_instructions}, codegen::generate_code,
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

    let instructions = decode_instructions(file_contents.as_str())?;
    let code_gen = generate_code(&instructions);

    println!("Instructions: {instructions:?}");

    Ok(ExitCode::from(0))
}

