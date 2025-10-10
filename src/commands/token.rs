use anyhow::{Error, anyhow};
use candid::Principal;
use clap::{Args, Parser, Subcommand};
use ic_agent::Agent;

use crate::{
    commands::{
        Context, Mode,
        args::{self, Validate, ValidateError},
    },
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

impl Validate for TransferArgs {
    fn validate(&self, mode: &Mode) -> Result<(), ValidateError> {
        match (&mode, self) {
            (
                Mode::Global,
                TransferArgs {
                    network: Some(args::Network::Name(_)),
                    ..
                },
            ) => Err(anyhow!("please provide a network url").into()),

            _ => Ok(()),
        }
    }
}

pub async fn transfer(ctx: &Context, args: &TransferArgs) -> Result<(), Error> {
    let (from, to) = match &ctx.mode {
        //
        Mode::Project(dir) => {
            todo!()
        }

        //
        Mode::Global => {
            todo!()
        }
    };

    let agent = Agent::builder().build()?;
    (ctx.ops.token.transfer)(&agent).transfer(from, to).await?;

    Ok(())
}
