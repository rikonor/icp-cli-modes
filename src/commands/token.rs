use anyhow::Error;
use candid::Principal;
use clap::{Args, Parser, Subcommand};

use crate::{
    commands::{Context, Mode, args},
    operations,
};

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
    let (from, to) = match &ctx.mode {
        //
        Mode::Project(dir) => todo!(),

        //
        Mode::Global => todo!(),
    };

    operations::token::transfer(from, to).await?;

    Ok(())
}
