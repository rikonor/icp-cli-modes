use clap::{Parser, Subcommand};

mod start;
pub use start::*;

mod stop;
pub use stop::*;

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
