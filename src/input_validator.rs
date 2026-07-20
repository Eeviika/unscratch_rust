use anyhow::{Ok, Result, anyhow, bail};
use std::{
    ffi::OsStr,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

pub fn derive_output(input: &Path) -> PathBuf {
    let mut out = input.to_path_buf();

    if let Some(stem) = input.file_stem() {
        out.set_file_name(format!("{}_out", stem.to_string_lossy()));
    } else {
        eprintln!("Warning: The input file has no name, so you should specify an output folder.");
        eprintln!("         Will output to a folder called \"unscratch_output\" instead.");
        out.set_file_name("unscratch_output");
    }

    out
}

pub fn validate_inputs(input: &Path, output: &Path, force: bool) -> Result<()> {
    if !input.exists() {
        return Err(anyhow!("The input file must exist."));
    }

    if !input.is_file() {
        return Err(anyhow!(
            "The input was expected to be a file, but we found a directory."
        ));
    }

    let ext = input.extension().unwrap_or(OsStr::new(""));

    if ext.is_empty() && !force {
        return Err(anyhow!(
            "The input file doesn't have an extension. Did you select the right file?"
        ));
    } else if ext.is_empty() {
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

    if output.is_file() {
        return Err(anyhow!(
            "The output folder specified is an already existing file."
        ));
    }

    if output.is_dir() && !force {
        return Err(anyhow!(
            "The output folder specified is an already existing folder."
        ));
    } else if output.is_dir() {
        println!(
            "Warning: Output folder specified already exists. Will delete because we are forcing file operations."
        )
    }

    Ok(())
}
