use anyhow::{Error, anyhow};
use candid::Principal;
use clap::Args;
use ic_agent::Agent;

use crate::commands::{
    Context, Mode,
    args::{self, Validate, ValidateError},
};

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
    // let cid = match &ctx.mode {
    //     //
    //     Mode::Project(dir) => {
    //         todo!()
    //     }

    //     //
    //     Mode::Global => {
    //         todo!()
    //     }
    // };

    let cid = Principal::anonymous();

    let agent = Agent::builder()
        .with_url("http://www,example.com")
        .build()?;

    (ctx.ops.canister.start)(&agent).start(&cid).await?;

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
            canister::{StartArgs, start},
        },
        operations::{
            self,
            canister::{self, MockStart},
        },
    };

    #[tokio::test]
    async fn start_in_project() -> Result<(), Error> {
        // Mode (Project)
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
