use clap::Parser as ClapParser;
use hack_assembler::{
    codegen::generate_code,
    instruction::Instruction,
    parser::{clean_line, decode_instruction, decode_instructions},
};
use std::{path::PathBuf, process::ExitCode, io::Write};

use anyhow::Result;

#[derive(ClapParser, Debug)]
struct Args {
    program: PathBuf,
}

fn main() -> Result<ExitCode> {
    let args = Args::parse();

    let extension = args.program.extension();
    if extension.is_none() || extension.unwrap() != "asm" {
        eprintln!("Input file must have .asm extension");
        return Ok(ExitCode::from(1));
    }

    if !args.program.is_file() {
        eprintln!("Failed to read input file {}", args.program.display());
        return Ok(ExitCode::from(2));
    }

    let file_contents = std::fs::read_to_string(&args.program)?;
    let lines = file_contents.lines();

    let instructions = decode_instructions(file_contents.as_str())?;
    let code_gen = generate_code(&instructions);

    println!("Instructions: {instructions:?}");

    let out_file_name = args.program.to_str().unwrap().replace(extension.unwrap().to_str().unwrap(), "hack");

    let mut out_file = std::fs::File::create(out_file_name)?;
    out_file.write_all(code_gen.as_bytes())?;

    Ok(ExitCode::from(0))
}
