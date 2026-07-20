use crate::cli::*;
use crate::input_validator::are_filepaths_ok;
use anyhow::{Ok, Result, anyhow, bail};
use clap::Parser;
use std::{ffi::OsStr, fs::File, io::BufReader, path::PathBuf};
use zip::ZipArchive;

use crate::scratch::scratch_structs::*;

pub fn unpack(input: PathBuf, output: PathBuf, as_is: bool, cli_options: CliOptions) -> Result<()> {
    println!("Beginning unpack...");

    are_filepaths_ok(&input, &output, cli_options.force)?;

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
