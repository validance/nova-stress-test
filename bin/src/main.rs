use crate::cli::{Cli, Commands};
use crate::worker::spawn_workers;

mod cli;
mod proto;

#[allow(clippy::too_many_arguments)]
mod worker;

use clap::{CommandFactory, Parser};
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::BuildProto {
            target_path,
            out_dir,
            include_dir,
        }) => proto::build_proto(&target_path, &out_dir, &include_dir),
        Some(Commands::Start { config }) => {
            let rt = tokio::runtime::Runtime::new()?;
            spawn_workers(config, rt)
        }
        None => {
            Cli::command().print_help().unwrap();
            process::exit(0);
        }
    }
    Ok(())
}
