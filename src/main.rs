mod cli;
mod input_validator;
mod scratch;
mod unpack;
mod unscratch;

use anyhow::{Ok, Result, bail};
use clap::Parser;
use cli::*;
use input_validator::derive_output;
use unpack::unpack;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cli_options = CliOptions::from_cli(&cli);

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
