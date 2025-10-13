use anyhow::{Error, anyhow};
use candid::Principal;
use clap::Args;
use ic_agent::Agent;

use crate::commands::{
    Context, Mode,
    args::{self, Validate, ValidateError},
};

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
            canister::{StopArgs, stop},
        },
        operations::{
            self,
            canister::{self, MockStop},
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
}
