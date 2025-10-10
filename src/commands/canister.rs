use anyhow::Error;
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
    Start(StartArgs),
    Stop(StopArgs),
}

#[derive(Args)]
pub struct StartArgs {
    canister: args::Canister,

    // Network
    #[arg(long)]
    network: Option<args::Network>,

    // Environment
    #[arg(long)]
    environment: Option<String>,
}

pub async fn start(ctx: &Context, args: &StartArgs) -> Result<(), Error> {
    let cid = match &ctx.mode {
        //
        Mode::Project(dir) => todo!(),

        //
        Mode::Global => todo!(),
    };

    operations::canister::start(cid).await?;

    Ok(())
}

#[derive(Args)]
pub struct StopArgs {
    canister: args::Canister,

    // Network
    network: Option<args::Network>,

    // Environment
    environment: Option<String>,
}

pub async fn stop(ctx: &Context, args: &StopArgs) -> Result<(), Error> {
    let cid = match &ctx.mode {
        //
        Mode::Project(dir) => todo!(),

        //
        Mode::Global => todo!(),
    };

    operations::canister::stop(cid).await?;

    Ok(())
}
