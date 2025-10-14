use anyhow::Error;
use candid::Principal;
use clap::Args;
use ic_agent::Agent;

use crate::{
    commands::{
        Context, Mode,
        args::{self, Validate, ValidateError, validations},
    },
    impl_from_args,
};

#[derive(Args)]
pub struct StopArgs {
    pub canister: args::Canister,

    // Network
    pub network: Option<args::Network>,

    // Environment
    pub environment: Option<String>,
}

impl_from_args!(StopArgs, canister: args::Canister);
impl_from_args!(StopArgs, network: Option<args::Network>);
impl_from_args!(StopArgs, environment: Option<String>);
impl_from_args!(StopArgs, network: Option<args::Network>, environment: Option<String>);

impl Validate for StopArgs {
    fn validate(&self, mode: &Mode) -> Result<(), ValidateError> {
        // Custom Tests
        for test in [
            //
            // first custom check
            |_args, _m| Some("zob".to_string()),
            //
            // second custom check
            |_args, _m| Some("butts".to_string()),
        ] {
            test(self, mode)
                .map(|msg| anyhow::format_err!(msg))
                .map_or(Ok(()), Err)?;
        }

        // General Tests
        for test in [
            validations::a_canister_id_is_required_in_global_mode,
            validations::a_network_name_is_required_in_project_mode,
            validations::a_network_url_is_required_in_global_mode,
            validations::environments_are_not_available_in_a_global_mode,
            validations::network_or_environment_not_both,
        ] {
            test(self, mode)
                .map(|msg| anyhow::format_err!(msg))
                .map_or(Ok(()), Err)?;
        }

        Ok(())
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
