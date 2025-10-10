use anyhow::{Error, bail};
use candid::Principal;
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
        Mode::Project(dir) => {
            if ![matches!(args.network, Some(args::Network::Name(_)))].all() {
                bail!("butt 1");
            }

            todo!()
        }

        //
        Mode::Global => {
            if ![matches!(args.network, Some(args::Network::Url(_)))].all() {
                bail!("butt 2");
            }

            todo!()
        }
    };

    operations::token::transfer(from, to).await?;

    Ok(())
}
