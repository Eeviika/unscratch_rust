mod cli;
mod input_validator;
mod scratch;
mod unpack;
mod unscratch;

use anyhow::{Ok, Result, bail};
use clap::Parser;
use cli::*;
use colog;
use input_validator::derive_output;
use log::error;

use crate::unpack::{UnpackArgs, unpack};

fn main() -> Result<()> {
    let mut clog = colog::default_builder();
    let cli = Cli::parse();
    let cli_options = CliOptions::from_cli(&cli);

    if cli_options.verbose && cli_options.silent {
        bail!("Verbose and Silent cannot be active at the same time.");
    }

    if cli_options.verbose {
        clog.filter(None, log::LevelFilter::Debug);
    }

    if cli_options.silent {
        clog.filter(None, log::LevelFilter::Off);
    }

    clog.init();

    let result = match cli.command {
        CommandType::Unpack {
            input,
            output,
            as_is,
            no_assets,
        } => {
            let output = output.unwrap_or_else(|| derive_output(&input));
            let args = UnpackArgs {
                input,
                output,
                as_is,
                no_assets,
            };
            unpack(args, cli_options)
        }
        _ => todo!("implement other subcommands"),
    };

    result
}
