use crate::cli::*;
use crate::input_validator::are_filepaths_ok;
use anyhow::{Ok, Result};
use file_format::{FileFormat, Kind};
use log::{debug, info, warn};
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

    debug!(
        "unpacking {} to {}\nas is? {as_is}\nwithout assets? {no_assets}",
        input.display(),
        output.display()
    );

    are_filepaths_ok(&input, &output, cli_options.force)?;

    info!("Beginning unpack...");

    let file = File::open(&input)?;
    let reader = BufReader::new(file);

    let mut archive = ZipArchive::new(reader)?;

    create_folders(&output)?;

    info!("Scanning project file...");

    let root_dir = archive.root_dir(zip::read::root_dir_common_filter)?;

    let project_path = match root_dir {
        Some(root) => {
            warn!(
                "Project file contains top-level directory, which is unusual.\nThis will not affect output."
            );
            root.join("project.json")
        }
        None => PathBuf::from("project.json"),
    };

    if !no_assets {
        info!("Exporting assets...");
        export_assets(&mut archive, &output)?;
        info!("Done!");
    }

    let json_file = archive.by_name(project_path.to_str().unwrap())?;

    info!("Deserializing project file (this may take a moment)...");
    let scratch_project: ScratchProject = serde_json::from_reader(json_file)?;
    info!("Done!");

    Ok(())
}

fn create_folders(output: &Path) -> Result<()> {
    if output.exists() {
        fs::remove_dir_all(output)?;
        debug!("removed {} as it already existed", output.display())
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
    info!("Created output directory.");
    Ok(())
}

fn export_assets<R>(archive: &mut ZipArchive<R>, output_root: &Path) -> Result<()>
where
    R: Read + Seek,
{
    let assets = output_root.join(ASSETS_FOLDERNAME);
    let sounds = assets.join(SOUNDS_FOLDERNAME);
    let costumes = assets.join(COSTUMES_FOLDERNAME);

    for i in 0..archive.len() {
        let mut archive_file = archive.by_index(i)?;

        if archive_file.is_dir() {
            debug!(
                "skipping exporting asset {} as it is a directory",
                archive_file.name()
            );
            continue;
        }

        let mut bytes = Vec::new();
        archive_file.read_to_end(&mut bytes)?;

        let format = FileFormat::from_bytes(&bytes);
        let kind = format.kind();

        if kind == Kind::Audio || kind == Kind::Image {
            debug!(
                "attempting to export asset {} as {kind:?}",
                archive_file.name()
            );
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

            let mut asset_file = File::create(&path)?;
            asset_file.write_all(&bytes)?;
            debug!(
                "exported asset {} as {kind:?} to {}",
                archive_file.name(),
                path.display()
            )
        }
    }

    return Ok(());
}
