mod cli;
mod proto;

use crate::cli::{Cli, Commands};
use clap::{CommandFactory, Parser};
use std::process;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::BuildProto {
            target_path,
            out_dir,
        }) => proto::build_proto(&target_path, &out_dir),
        Some(Commands::Start {}) => {}
        None => {
            Cli::command().print_help().unwrap();
            process::exit(0);
        }
    }
}
