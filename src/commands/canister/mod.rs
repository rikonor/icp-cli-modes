use clap::{Parser, Subcommand};

use crate::commands::canister::{start::StartArgs, stop::StopArgs};

pub mod start;
pub mod stop;

#[derive(Parser)]
pub struct Command {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start(StartArgs),
    Stop(StopArgs),
}
