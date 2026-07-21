use anyhow::{Ok, Result, anyhow};
use log::{debug, warn};
use std::path::{Path, PathBuf};

fn check(condition: bool, warning: &str, error: &str, force: bool) -> Result<()> {
    if condition {
        return Ok(());
    } else if force {
        warn!("{warning}");
        return Ok(());
    }

    Err(anyhow!("{error}"))
}

pub fn derive_output(input: &Path) -> PathBuf {
    debug!("deriving output from {}", input.display());
    let mut out = input.to_path_buf();

    if let Some(stem) = input.file_stem() {
        let name = format!("{}_out", stem.to_string_lossy());
        debug!("derived output as {name}");
        out.set_file_name(name);
    } else {
        warn!(
            "The input file has no name, so you should specify an output folder.\n
            Will output to a folder called \"unscratch_output\" instead."
        );
        out.set_file_name("unscratch_output");
    }

    out
}

pub fn are_filepaths_ok(input: &Path, output: &Path, force: bool) -> Result<()> {
    check(input.exists(), "", "Input file does not exist.", false)?;
    check(input.is_file(), "", "Input must be a file.", false)?;
    check(
        !output.is_file(),
        "",
        "Output points to an existing file.",
        false,
    )?;
    check(
        !output.is_dir(),
        "Output is existing directory, overwriting.",
        "Output points to an existing directory. Stopping.\n(Use --force to overwrite).",
        force,
    )?;

    Ok(())
}
