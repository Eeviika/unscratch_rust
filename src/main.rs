mod scratch;
mod unscratch;
mod cli;

use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};
use cli::*;

const LONG_ABOUT: &str = "A program / CLI tool to pack and unpack Scratch project files on your filesystem.\nAuthor: Eeviika";

fn main() {
    let cli = CLI::parse();
    let is_verbose = cli.verbose;

    match cli.command {
        CommandType::Unpack { input, output, dry_run } => unpack(input, output, dry_run),
        _ => { todo!("Only Unpack is implemented at this time") }
    }
}

fn unpack(input: PathBuf, output: Option<PathBuf>, dry_run: bool) {
    if !input.exists() {
        panic!("input file must exist")
    }
}
