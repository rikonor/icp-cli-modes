use anyhow::Error;
use candid::Principal;
use clap::{Args, Parser, Subcommand};

use crate::commands::{Context, args};

#[derive(Parser)]
pub struct Command {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Transfer(TransferArgs),
}

#[derive(Args)]
pub struct TransferArgs {
    from: Principal,
    to: Principal,

    #[arg(long)]
    network: Option<args::Network>,
}

pub async fn transfer(ctx: &Context, args: &TransferArgs) -> Result<(), Error> {
    Ok(())
}
