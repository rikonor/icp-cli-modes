use anyhow::{Error, bail};
use clap::{Args, Parser, Subcommand};

use crate::{
    commands::{BoolSliceExt, Context, Mode, args},
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
        Mode::Project(dir) => {
            if ![
                matches!(args.canister, args::Canister::Name(_)),
                matches!(args.network, Some(args::Network::Name(_))),
            ]
            .all()
            {
                bail!("butt");
            }

            todo!()
        }

        //
        Mode::Global => {
            if ![
                matches!(args.canister, args::Canister::Principal(_)),
                matches!(args.network, Some(args::Network::Url(_))),
            ]
            .all()
            {
                bail!("butt");
            }

            todo!()
        }
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
