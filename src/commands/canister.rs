use anyhow::{Error, anyhow};
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

impl Validate for StartArgs {
    fn validate(&self, mode: &Mode) -> Result<(), ValidateError> {
        match (&mode, self) {
            (
                Mode::Project(_),
                StartArgs {
                    network: Some(_),
                    environment: Some(_),
                    ..
                },
            ) => Err(anyhow!("not allowed to have both network and environment").into()),

            (
                Mode::Global,
                StartArgs {
                    environment: Some(_),
                    ..
                },
            ) => Err(anyhow!("environments are not available in a global context").into()),

            (
                Mode::Global,
                StartArgs {
                    network: Some(args::Network::Name(_)),
                    ..
                },
            ) => Err(anyhow!("please provide a network url").into()),

            _ => Ok(()),
        }
    }
}

pub async fn start(ctx: &Context, args: &StartArgs) -> Result<(), Error> {
    let cid = match &ctx.mode {
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
    (ctx.ops.canister.start)(&agent).start(cid).await?;

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

impl Validate for StopArgs {
    fn validate(&self, mode: &Mode) -> Result<(), ValidateError> {
        match (&mode, self) {
            (
                Mode::Project(_),
                StopArgs {
                    network: Some(_),
                    environment: Some(_),
                    ..
                },
            ) => Err(anyhow!("not allowed to have both network and environment").into()),

            (
                Mode::Global,
                StopArgs {
                    environment: Some(_),
                    ..
                },
            ) => Err(anyhow!("environments are not available in a global context").into()),

            (
                Mode::Global,
                StopArgs {
                    network: Some(args::Network::Name(_)),
                    ..
                },
            ) => Err(anyhow!("please provide a network url").into()),

            _ => Ok(()),
        }
    }
}

pub async fn stop(ctx: &Context, args: &StopArgs) -> Result<(), Error> {
    let cid = match &ctx.mode {
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
    (ctx.ops.canister.stop)(&agent).stop(cid).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO: tests for args validation
}
