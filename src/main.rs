mod cli;
mod scratch;
mod unscratch;

use anyhow::{Ok, Result, anyhow, bail};
use clap::Parser;
use cli::*;
use log::debug;
use std::{fs::File, io::BufReader, path::PathBuf};
use zip::extra_fields;

use crate::scratch::scratch_structs::ScratchProject;

fn main() -> Result<()> {
    debug!("Parsing...");
    let cli = CLI::parse();
    let is_verbose = cli.verbose;

    match cli.command {
        CommandType::Unpack {
            input,
            output,
            dry_run,
        } => unpack(input, output, dry_run, is_verbose)?,
        _ => bail!("not implemented"),
    }

    Ok(())
}

fn unpack(input: PathBuf, output: Option<PathBuf>, dry_run: bool, is_verbose: bool) -> Result<()> {
    if !input.exists() {
        return Err(anyhow!("the input file must exist"));
    }

    if !input.is_file() {
        return Err(anyhow!("the input is not a file; but is a directory"));
    }

    if input.file_stem().is_none() && output.is_none() {
        return Err(anyhow!(
            "input file has no name, please define output folder"
        ));
    }

    let ext = input.extension();

    if ext.is_none() {
        return Err(anyhow!("the input file has no extension"));
    }

    if ext.unwrap() != "sb3" {
        return Err(anyhow!("the input file is not a scratch project (.sb3)"));
    }

    let output = output.unwrap_or_else(|| {
        let stem = input.file_stem().expect("input has no file stem");
        PathBuf::from(stem)
    });

    let file = File::open(&input)?;
    let reader = BufReader::new(file);
    let scratch_project: ScratchProject = serde_json::from_reader(reader)?;

    Ok(())
}
