use crate::cli::*;
use crate::input_validator::are_filepaths_ok;
use anyhow::{Ok, Result};
use file_format::{FileFormat, Kind};
use std::{
    fs::{self, File},
    io::{BufReader, Read, Seek, Write},
    path::{Path, PathBuf},
};
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

    create_folders(&output)?;

    println!("Scanning project file...");

    let mut archive = ZipArchive::new(reader)?;

    if !no_assets {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_message("Exporting assets...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        export_assets(&mut archive, &output)?;
        pb.finish();
    }

    let json_file = archive.by_name("project.json")?;

    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_message("Parsing project JSON...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let scratch_project: ScratchProject = serde_json::from_reader(json_file)?;

    pb.finish();

    Ok(())
}

fn create_folders(output: &Path) -> Result<()> {
    if output.exists() {
        fs::remove_dir_all(&output)?;
    }

    let assets = output.join(ASSETS_FOLDERNAME);
    let sounds = assets.join(SOUNDS_FOLDERNAME);
    let costumes = assets.join(COSTUMES_FOLDERNAME);
    let sprites = output.join(SPRITES_FOLDERNAME);
    let scripts = output.join(SCRIPTS_FOLDERNAME);

    fs::create_dir_all(assets)?;
    fs::create_dir_all(sounds)?;
    fs::create_dir_all(costumes)?;
    fs::create_dir_all(sprites)?;
    fs::create_dir_all(scripts)?;
    Ok(())
}

fn export_assets<R>(archive: &mut ZipArchive<R>, output_root: &Path) -> Result<()>
where
    R: Read + Seek,
{
    let mut valid_filenames = Vec::new();

    let assets = output_root.join(ASSETS_FOLDERNAME);
    let sounds = assets.join(SOUNDS_FOLDERNAME);
    let costumes = assets.join(COSTUMES_FOLDERNAME);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        if file.is_dir() {
            continue;
        }

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        let format = FileFormat::from_bytes(&bytes);
        let kind = format.kind();

        if kind == Kind::Audio || kind == Kind::Image {
            let name = file.name().to_owned();
            // println!("{}", name);
            valid_filenames.push(name);
        }
    }

    for filename in valid_filenames {
        let mut archive_file = archive.by_name(&filename)?;

        let mut bytes = Vec::new();
        archive_file.read_to_end(&mut bytes)?;

        let format = FileFormat::from_bytes(&bytes);
        let kind = format.kind();

        let opt_path = archive_file.enclosed_name().to_owned();

        if opt_path.is_none() {
            continue;
        }

        let enclosed_path = opt_path.unwrap();
        let opt_name = enclosed_path.file_name();

        if opt_name.is_none() {
            continue;
        }

        let enclosed_name = opt_name.unwrap();
        let path = match kind {
            Kind::Image => costumes.join(enclosed_name),
            Kind::Audio => sounds.join(enclosed_name),
            _ => assets.join(enclosed_name),
        };

        let mut asset_file = File::create(path)?;
        asset_file.write_all(&bytes)?;
    }

    return Ok(());
}
