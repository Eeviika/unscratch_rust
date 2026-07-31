use crate::input_validator::are_filepaths_ok;
use crate::{cli::*, unscratch::sprite::Sprite};
use anyhow::Result;
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
const DATA_FOLDERNAME: &str = "data";

pub struct UnpackArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    pub as_is: bool,
    pub no_assets: bool,
}

pub fn unpack(args: UnpackArgs, cli_options: CliOptions) -> Result<()> {
    let UnpackArgs {
        input,
        output,
        as_is,
        no_assets,
    } = args;

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

    create_output_tree(&output)?;

    info!("Scanning project file...");

    if !no_assets {
        info!("Exporting assets...");
        export_assets(&mut archive, &output)?;
        info!("Done!");
    }

    export_project(&mut archive, &output, as_is)?;

    Ok(())
}

fn create_output_tree(output: &Path) -> Result<()> {
    if output.exists() {
        fs::remove_dir_all(output)?;
        debug!("removed {} as it already existed", output.display());
    }

    let assets = output.join(ASSETS_FOLDERNAME);
    let sounds = assets.join(SOUNDS_FOLDERNAME);
    let costumes = assets.join(COSTUMES_FOLDERNAME);
    let sprites = output.join(SPRITES_FOLDERNAME);
    let scripts = output.join(SCRIPTS_FOLDERNAME);
    let data = output.join(DATA_FOLDERNAME);

    for directory in [assets, sounds, costumes, sprites, scripts, data] {
        fs::create_dir_all(directory)?;
    }

    info!("Created output directory tree.");
    Ok(())
}

fn deserialize_project<R>(archive: &mut ZipArchive<R>) -> Result<ScratchProject>
where
    R: Read + Seek,
{
    debug!("Preparing to deserialize project file...");
    let project_path = project_json_path(archive)?;

    info!("Deserializing project file (this may take a while)...");
    let mut json_file = archive.by_name(project_path.to_string_lossy().as_ref())?;
    let mut json_string = String::new();
    json_file.read_to_string(&mut json_string)?;
    let result = serde_json::from_str(&json_string)?;
    info!("Done!");
    Ok(result)
}

fn project_json_path<R>(archive: &ZipArchive<R>) -> Result<PathBuf>
where
    R: Read + Seek,
{
    match archive.root_dir(zip::read::root_dir_common_filter)? {
        Some(root) => {
            warn!(
                "Project file contains top-level directory, which is unusual.\nThis will not affect output."
            );
            debug!("Root is {}", root.display());
            Ok(root.join("project.json"))
        }
        None => Ok(PathBuf::from("project.json")),
    }
}

fn export_project<R>(archive: &mut ZipArchive<R>, output: &Path, as_is: bool) -> Result<()>
where
    R: Read + Seek,
{
    info!("Exporting project...");
    let project = deserialize_project(archive)?;
    let meta = project.meta;

    debug!("{meta:#?}");
    debug!("Extensions:     {:?}", project.extensions);
    debug!("Extension URLs: {:?}", project.extension_urls);

    info!("Exporting sprites...");
    if as_is {
        for target in project.targets {
            export_sprite_as_is(target, &output)?;
        }
    } else {
        for target in project.targets {
            export_reformatted_sprite(target, &output)?;
        }
    }

    Ok(())
}

fn export_reformatted_sprite(target: ScratchTarget, sprites_path: &Path) -> Result<()> {
    let sprite_name = &target.name;

    if target.variables.is_empty() && target.lists.is_empty() && target.blocks.is_empty() {
        warn!("Not exporting sprite {} as it is blank.", sprite_name);
        return Ok(());
    }

    let path = sprites_path.join(format!("{sprite_name}.toml"));
    debug!(
        "attempting to export sprite {} as reformatted TOML to {}",
        sprite_name,
        path.display()
    );

    let sprite = Sprite::try_from(target)?;
    let toml = toml::to_string_pretty(&sprite)?;
    let mut file = File::create(path)?;
    file.write_all(toml.as_bytes())?;

    Ok(())
}

fn export_sprite_as_is(target: ScratchTarget, output: &Path) -> Result<()> {
    let sprites_path = output.join(SPRITES_FOLDERNAME);
    let lists_path = output.join(DATA_FOLDERNAME);

    let sprite_name = &target.name;

    if target.variables.is_empty() && target.lists.is_empty() && target.blocks.is_empty() {
        warn!("Not exporting sprite {} as it is blank.", sprite_name);
        return Ok(());
    }

    let sprite_path = sprites_path.join(format!("{sprite_name}.json"));
    let list_path = lists_path.join(format!("{sprite_name}.lists.json"));

    debug!(
        "attempting to export sprite {} as JSON to {}",
        sprite_name,
        sprite_path.display()
    );

    let sprite_json = serde_json::to_string_pretty(&target)?;
    let mut file = File::create(sprite_path)?;
    file.write_all(sprite_json.as_bytes())?;

    let list_json = serde_json::to_string_pretty(&target.lists)?;
    let mut file = File::create(list_path)?;
    file.write_all(list_json.as_bytes())?;

    Ok(())
}

fn export_assets<R>(archive: &mut ZipArchive<R>, output: &Path) -> Result<()>
where
    R: Read + Seek,
{
    let assets = output.join(ASSETS_FOLDERNAME);
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

        let kind = FileFormat::from_bytes(&bytes).kind();

        if matches!(kind, Kind::Audio | Kind::Image) {
            debug!("attempting to export asset {name} as {kind:?}");

            let Some(enclosed_path) = archive_file.enclosed_name() else {
                continue;
            };
            let Some(file_name) = enclosed_path.file_name() else {
                continue;
            };

            let path = match kind {
                Kind::Image => costumes.join(file_name),
                Kind::Audio => sounds.join(file_name),
                _ => assets.join(file_name),
            };

            let mut asset_file = File::create(&path)?;
            asset_file.write_all(&bytes)?;
            debug!("exported asset {name} as {kind:?} to {}", path.display());
        }
    }

    Ok(())
}
