use anyhow::Error;
use candid::Principal;
use clap::Args;
use ic_agent::Agent;
use indoc::formatdoc;

use crate::commands::{
    Context, Mode,
    args::{self, Network, Validate, ValidateError},
};

#[derive(Args, Clone)]
pub struct StartArgs {
    canister: args::Canister,

    // Network
    #[arg(long)]
    network: Option<args::Network>,

    // Environment
    #[arg(long)]
    environment: Option<String>,
}

impl<'a> From<&'a StartArgs> for (&'a Option<Network>, &'a Option<String>) {
    fn from(args: &'a StartArgs) -> Self {
        (&args.network, &args.environment)
    }
}

impl<'a> From<&'a StartArgs> for (&'a Option<Network>,) {
    fn from(args: &'a StartArgs) -> Self {
        (&args.network,)
    }
}

impl<'a> From<&'a StartArgs> for (&'a Option<String>,) {
    fn from(args: &'a StartArgs) -> Self {
        (&args.environment,)
    }
}

fn network_or_environment_not_both<'a>(
    network_environment: impl Into<(&'a Option<Network>, &'a Option<String>)>,
    m: &Mode,
) -> Option<String> {
    let (network, environment) = network_environment.into();

    (matches!(m, _) && network.is_some() && environment.is_some()).then_some(formatdoc! {"
        Please provide either a network or an environment, but not both.
    "})
}

fn environments_are_not_available_in_a_global_mode<'a>(
    environment: impl Into<(&'a Option<String>,)>,
    m: &Mode,
) -> Option<String> {
    let (environment,) = environment.into();

    (matches!(m, Mode::Global) && environment.is_some()).then_some(formatdoc! {"
        Environments are not available in global mode.
    "})
}

fn a_network_url_is_required_in_global_mode<'a>(
    network: impl Into<(&'a Option<Network>,)>,
    m: &Mode,
) -> Option<String> {
    let (network,) = network.into();

    (matches!(m, Mode::Global) && !matches!(network, Some(Network::Url(_)))).then_some(
        formatdoc! {"
            A network `url` is required in global mode.
        "},
    )
}

fn a_network_name_is_required_in_project_mode<'a>(
    network: impl Into<(&'a Option<Network>,)>,
    m: &Mode,
) -> Option<String> {
    let (network,) = network.into();

    (matches!(m, Mode::Project(_)) && !matches!(network, Some(Network::Name(_)))).then_some(
        formatdoc! {"
            A network `name` is required in project mode.
        "},
    )
}

impl Validate for StartArgs {
    fn validate(&self, mode: &Mode) -> Result<(), ValidateError> {
        // General Tests
        for test in [
            a_network_name_is_required_in_project_mode,
            a_network_url_is_required_in_global_mode,
            environments_are_not_available_in_a_global_mode,
            network_or_environment_not_both,
        ] {
            test(self, mode)
                .map(|msg| anyhow::format_err!(msg))
                .map_or(Ok(()), Err)?;
        }

        // Custom Tests
        for test in [
            //
            // first custom check
            |_args, _m| Some("butts".to_string()),
            //
            // second custom check
            |_args, _m| Some("butts".to_string()),
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
