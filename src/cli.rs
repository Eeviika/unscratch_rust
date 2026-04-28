use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

const LONG_ABOUT: &str = "A CLI tool to pack and unpack Scratch project files.";

#[derive(Debug, Clone, ValueEnum)]
pub enum ProjectType {
    Unscratch,
    Scratch,
}

#[derive(Subcommand)]
pub enum CommandType {
    /// Unpack a Scratch project onto the filesystem
    Unpack {
        /// The path to the Scratch project
        #[arg(short, long)]
        input: PathBuf,
        /// The path to the folder to unpack the project to
        /// (It will be created if it doesn't exist)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Force file operations, ignoring safety checks
        #[arg(short, long)]
        force: bool,
        /// Do not perform any file operations
        #[arg(long)]
        dry_run: bool,
        /// Export the project with no tweaks applied
        #[arg(long)]
        as_is: bool,
    },
    /// Pack an unpacked project back into a Scratch project
    Pack {
        /// Path to the unpacked project folder
        #[arg(short, long)]
        input: PathBuf,
        /// Filename / path to output project file to
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Do not perform any file operations
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = LONG_ABOUT, arg_required_else_help = true)]
pub struct CLI {
    /// Tell unscratch to print more info
    #[arg(short, long)]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: CommandType,
}
