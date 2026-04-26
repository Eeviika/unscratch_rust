use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

const LONG_ABOUT: &str = "a cli tool to pack and unpack scratch project files";

#[derive(Debug, Clone, ValueEnum)]
pub enum ProjectType {
    Unscratch,
    Scratch,
}

#[derive(Subcommand)]
pub enum CommandType {
    /// unpack a scratch project onto the filesystem
    Unpack {
        /// path to scratch project
        #[arg(short, long)]
        input: PathBuf,
        /// path to folder to unpack the project to (will be created)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// don't actually do anything
        #[arg(long)]
        dry_run: bool,
    },
    /// pack an unscratch project into a scratch project
    Pack {
        /// path to unpacked (unscratch) project
        #[arg(short, long)]
        input: PathBuf,
        /// path to pack scratch project to
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// don't actually do anything
        #[arg(long)]
        dry_run: bool,
    },
    /// check a project for validity
    Verify {
        /// path to project
        #[arg(short, long)]
        input: PathBuf,
        /// force a certain project type
        #[arg(short, long)]
        force_type: Option<ProjectType>,
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
