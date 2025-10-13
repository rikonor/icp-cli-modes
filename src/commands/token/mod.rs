use clap::{Parser, Subcommand};

use crate::commands::token::transfer::TransferArgs;

pub mod transfer;

#[derive(Parser)]
pub struct Command {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Transfer(TransferArgs),
}
