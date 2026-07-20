use crate::cli::*;
use crate::input_validator::are_filepaths_ok;
use anyhow::{Ok, Result};
use std::{fs::File, io::BufReader, path::PathBuf};
use zip::ZipArchive;

use crate::scratch::scratch_structs::*;

const ASSETS_FOLDERNAME: &str = "assets";
const SOUNDS_FOLDERNAME: &str = "sounds";
const COSTUMES_FOLDERNAME: &str = "costumes";
const SPRITES_FOLDERNAME: &str = "sprites";
const SCRIPTS_FOLDERNAME: &str = "scripts";

pub struct UnpackArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    pub as_is: bool,
    pub no_assets: bool,
}

pub fn unpack(args: UnpackArgs, cli_options: CliOptions) -> Result<()> {
    let input = args.input;
    let output = args.output;
    let as_is = args.as_is;
    let no_assets = args.no_assets;

    are_filepaths_ok(&input, &output, cli_options.force)?;

    println!("Beginning unpack...");

    let file = File::open(&input)?;
    let reader = BufReader::new(file);

    println!("Scanning project file...");

    let mut archive = ZipArchive::new(reader)?;

    let json_file = archive.by_name("project.json")?;

    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_message("Parsing project file...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let scratch_project: ScratchProject = serde_json::from_reader(json_file)?;

    pb.finish_with_message("Parsing project file... OK!");

    Ok(())
}
