mod cli;
mod scratch;
mod unscratch;

use anyhow::{Ok, Result, anyhow, bail};
use clap::Parser;
use cli::*;
use std::{ffi::OsStr, fs::File, io::BufReader, path::PathBuf};
use zip::ZipArchive;

use crate::scratch::scratch_structs::ScratchProject;

fn main() -> Result<()> {
    let cli = CLI::parse();
    let is_verbose = cli.verbose;

    match cli.command {
        CommandType::Unpack {
            input,
            output,
            force,
            dry_run,
            as_is,
        } => {
            let out = output.unwrap_or_else(|| derive_output(&input));
            unpack(input, out, force, dry_run, as_is, is_verbose)?
        }
        _ => bail!("not implemented"),
    }

    Ok(())
}

fn derive_output(input: &PathBuf) -> PathBuf {
    let mut out = input.clone();

    if let Some(stem) = input.file_stem() {
        out.set_file_name(format!("{}_out", stem.to_string_lossy()));
    } else {
        println!("Warning: The input file has no name, so you should specify an output folder.");
        println!("         Will output to the \"./unscratch_output\" folder instead.");
        out.set_file_name("unscratch_output");
    }

    out
}

fn validate_inputs(input: &PathBuf, output: &PathBuf, force: bool) -> Result<()> {
    if !input.exists() {
        return Err(anyhow!("The input file must exist."));
    }

    if !input.is_file() {
        return Err(anyhow!(
            "The input was expected to be a file, but we found a directory."
        ));
    }

    let ext = input.extension().unwrap_or(OsStr::new(""));

    if ext == "" && !force {
        return Err(anyhow!(
            "The input file doesn't have an extension. Did you select the right file?"
        ));
    } else if ext == "" {
        println!(
            "The input file doesn't have an extension. Ignoring because we are forcing file operations."
        )
    }

    if ext != "sb3" && !force {
        return Err(anyhow!(
            "The input file doesn't have an \".sb3\" extension, did you select the right file? Unscratch only supports Scratch 3."
        ));
    } else if ext != "sb3" {
        println!(
            "Warning: The input file doesn't have the \".sb3\" extension. Ignoring because we are forcing file operations."
        );
    }

    Ok(())
}

fn unpack(
    input: PathBuf,
    output: PathBuf,
    force: bool,
    dry_run: bool,
    as_is: bool,
    is_verbose: bool,
) -> Result<()> {
    println!("Beginning unpack...");

    validate_inputs(&input, &output, force)?;

    println!("Got project file.");

    let file = File::open(&input)?;
    let reader = BufReader::new(file);

    println!("Scanning project file...");

    let mut archive = ZipArchive::new(reader)?;

    let json_file = archive.by_name("project.json")?;

    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_message("Parsing project file...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let scratch_project: ScratchProject = serde_json::from_reader(json_file)?;

    pb.finish_with_message("Parsing project file... Done!");

    Ok(())
}
