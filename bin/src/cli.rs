use clap::{Parser, Subcommand};

/// stress test for cosmos app-chains
#[derive(Parser, Debug)]
#[command(about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// start stress test
    Start {},

    /// compile proto
    BuildProto {
        #[arg(short, long)]
        target_path: String,

        #[arg(short, long)]
        out_dir: String,

        #[arg(short, long, action = clap::ArgAction::Append)]
        include_dir: Vec<String>,
    },
}
