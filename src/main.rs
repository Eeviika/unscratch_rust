#![allow(unused)]

mod cli;
mod input_validator;
mod scratch;
mod unpack;
mod unscratch;

use anyhow::{Ok, Result, anyhow, bail};
use clap::Parser;
use cli::*;
use input_validator::derive_output;
use std::{ffi::OsStr, fs::File, io::BufReader, path::PathBuf};
use unpack::unpack;
use zip::ZipArchive;

use crate::scratch::scratch_structs::ScratchProject;

fn main() -> Result<()> {
    let cli = CLI::parse();
    let cli_options = CLIOptions::from_cli(&cli);

    match cli.command {
        CommandType::Unpack {
            input,
            output,
            as_is,
            no_assets,
        } => {
            let output = output.unwrap_or_else(|| derive_output(&input));
            unpack(input, output, as_is, cli_options)?
        }
        _ => bail!("not implemented"),
    }

    Ok(())
}
