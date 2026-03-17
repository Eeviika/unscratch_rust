mod scratch;
mod unscratch;

use clap::Parser;
use std::path::PathBuf;

const LONG_ABOUT: &str = "A program / CLI tool to pack and unpack Scratch project files on your filesystem.\nAuthor: Eeviika";

#[derive(Parser)]
#[command(version, about, long_about = LONG_ABOUT)]
struct CLI {
    from: PathBuf,
    to: PathBuf,
}

fn main() {
    println!("Unscratch is in DEVELOPMENT, bugs may occur...");
    let cli = CLI::parse();
    println!("Goodbye, world!")
}
