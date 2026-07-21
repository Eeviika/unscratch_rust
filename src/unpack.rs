use crate::cli::*;
use crate::input_validator::are_filepaths_ok;
use anyhow::{Ok, Result, bail};
use file_format::{FileFormat, Kind};
use log::{debug, info, warn};
use std::{
    fs::{self, File},
    io::{BufReader, Error, Read, Seek, Write},
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

    if !no_assets {
        info!("Exporting assets...");
        export_assets(&mut archive, &output)?;
        info!("Done!");
    }

    export_project(&mut archive)?;

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

fn deserialize_project<R>(archive: &mut ZipArchive<R>) -> Result<ScratchProject>
where
    R: Read + Seek,
{
    debug!("Preparing to deserialize...");
    let root_dir = archive.root_dir(zip::read::root_dir_common_filter)?;

    let project_path = match root_dir {
        Some(root) => {
            warn!(
                "Project file contains top-level directory, which is unusual.\nThis will not affect output."
            );
            debug!("Root is {}", root.display());
            root.join("project.json")
        }
        None => PathBuf::from("project.json"),
    };

    info!("Deserializing project file (this may take a while)...");
    let mut json_file = archive.by_name(project_path.to_str().unwrap())?;
    let mut json_string = String::new();
    json_file.read_to_string(&mut json_string)?;
    let result: ScratchProject = serde_json::from_str(&json_string)?;
    info!("Done!");
    Ok(result)
}

fn export_project<R>(archive: &mut ZipArchive<R>) -> Result<()>
where
    R: Read + Seek,
{
    info!("Exporting project...");
    let project = deserialize_project(archive)?;

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
        let name = archive_file.name().to_owned();

        if archive_file.is_dir() {
            debug!("skipping exporting asset {name} as it is a directory",);
            continue;
        }

        let mut bytes = Vec::new();
        archive_file.read_to_end(&mut bytes)?;

        let format = FileFormat::from_bytes(&bytes);
        let kind = format.kind();

        if kind == Kind::Audio || kind == Kind::Image {
            debug!("attempting to export asset {name} as {kind:?}",);
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
            debug!("exported asset {name} as {kind:?} to {}", path.display())
        }
    }

    return Ok(());
}
