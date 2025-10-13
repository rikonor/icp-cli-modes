use anyhow::{Error, anyhow};
use candid::Principal;
use clap::{Args, Parser, Subcommand};
use ic_agent::Agent;

use crate::commands::{
    Context, Mode,
    args::{self, Validate, ValidateError},
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

    let cid = Principal::anonymous();

    let agent = Agent::builder()
        .with_url("http://www.example.com")
        .build()?;

    (ctx.ops.canister.stop)(&agent).stop(&cid).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Error;
    use candid::Principal;
    use mockall::predicate::eq;

    use crate::{
        commands::{
            Context, Mode, args,
            canister::{StartArgs, StopArgs, start, stop},
        },
        operations::{
            self,
            canister::{self, MockStart, MockStop},
        },
    };

    #[tokio::test]
    async fn stop_in_project() -> Result<(), Error> {
        // Mode (Project)
        let mode = Mode::Project("path".into());

        // Operations
        let ops = operations::Initializers {
            canister: canister::Initializers {
                stop: Box::new(|_| {
                    let mut m = MockStop::new();
                    m.expect_stop()
                        .with(eq(Principal::anonymous()))
                        .once()
                        .returning(|_| Ok(()));

                    Arc::new(m)
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let ctx = Context { mode, ops };

        let args = StopArgs {
            canister: args::Canister::Name("my-canister".to_string()),
            network: Some(args::Network::Name("my-network".to_string())),
            environment: None,
        };

        stop(&ctx, &args).await?;

        Ok(())
    }

    #[tokio::test]
    async fn stop_in_global() -> Result<(), Error> {
        // Mode (Global)
        let mode = Mode::Project("path".into());

        // Operations
        let ops = operations::Initializers {
            canister: canister::Initializers {
                start: Box::new(|_| {
                    let mut m = MockStart::new();
                    m.expect_start()
                        .with(eq(Principal::anonymous()))
                        .once()
                        .returning(|_| Ok(()));

                    Arc::new(m)
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let ctx = Context { mode, ops };

        let args = StartArgs {
            canister: args::Canister::Name("my-canister".to_string()),
            network: Some(args::Network::Name("my-network".to_string())),
            environment: None,
        };

        start(&ctx, &args).await?;

        Ok(())
    }
}
