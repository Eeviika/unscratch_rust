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
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Export the project as raw JSON files
        #[arg(long)]
        as_is: bool,
        /// Do not export costumes or sounds; removing all references to them
        #[arg(long)]
        no_assets: bool,
    },
    /// Pack an unpacked project back into a Scratch project
    Pack {
        /// Path to the unpacked project folder
        #[arg(short, long)]
        input: PathBuf,
        /// Filename / path to output project file to
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = LONG_ABOUT, arg_required_else_help = true)]
pub struct CLI {
    /// Tell Unscratch to print more info
    #[arg(short, long)]
    pub verbose: bool,
    /// Ignore file safety checks
    #[arg(short, long)]
    pub force: bool,
    /// Do not perform any file operations
    #[arg(short, long)]
    pub dry_run: bool,
    /// Do not output any logs
    #[arg(short, long)]
    pub silent: bool,
    #[command(subcommand)]
    pub command: CommandType,
}

pub struct CLIOptions {
    pub verbose: bool,
    pub force: bool,
    pub dry_run: bool,
}

impl CLIOptions {
    pub fn from_cli(cli: &CLI) -> Self {
        CLIOptions {
            verbose: cli.verbose,
            force: cli.force,
            dry_run: cli.dry_run,
        }
    }
}
