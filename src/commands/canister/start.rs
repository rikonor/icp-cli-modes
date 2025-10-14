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

#[derive(Args, Clone)]
pub struct StartArgs {
    pub canister: args::Canister,

    // Network
    #[arg(long)]
    pub network: Option<args::Network>,

    // Environment
    #[arg(long)]
    pub environment: Option<String>,
}

impl_from_args!(StartArgs, canister: args::Canister);
impl_from_args!(StartArgs, network: Option<args::Network>);
impl_from_args!(StartArgs, environment: Option<String>);
impl_from_args!(StartArgs, network: Option<args::Network>, environment: Option<String>);

impl Validate for StartArgs {
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
mod tests_args {
    use crate::commands::{
        Mode,
        args::{Canister, Network, Validate},
        canister::StartArgs,
    };

    #[test]
    fn args_valid() {
        let args = [
            (
                StartArgs {
                    canister: Canister::Name("my-canister".to_string()),
                    network: None,
                    environment: None,
                },
                vec![Mode::Global, Mode::Project("dir".into())],
            ),
            (
                StartArgs {
                    canister: Canister::Name("my-canister".to_string()),
                    network: None,
                    environment: None,
                },
                vec![Mode::Project("dir".into())],
            ),
        ];

        for (v, ms) in args {
            for m in ms {
                (v).validate(&m).expect("expected valid args");
            }
        }
    }

    #[test]
    fn args_invalid() {
        let args = [
            (
                "please provide a network url",
                StartArgs {
                    canister: Canister::Name("my-canister".to_string()),
                    network: Some(Network::Name("my-network".to_string())),
                    environment: None,
                },
                vec![Mode::Global],
            ),
            (
                "please provide a network name",
                StartArgs {
                    canister: Canister::Name("my-canister".to_string()),
                    network: Some(Network::Url("http://www.example.com".to_string())),
                    environment: None,
                },
                vec![Mode::Project("dir".into())],
            ),
            (
                "not allowed to have both network and environment",
                StartArgs {
                    canister: Canister::Name("my-canister".to_string()),
                    network: Some(Network::Url("http://www.example.com".to_string())),
                    environment: Some("my-environment".to_string()),
                },
                vec![Mode::Global, Mode::Project("dir".into())],
            ),
        ];

        for (msg, v, ms) in args {
            for m in ms {
                match (v).validate(&m) {
                    Ok(_) => panic!("expected invalid args: {msg}"),
                    Err(err) => assert_eq!(err.to_string(), msg),
                };
            }
        }
    }
}

#[cfg(test)]
mod tests_start {
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
