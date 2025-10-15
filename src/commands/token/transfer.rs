use candid::Principal;
use clap::Args;
use ic_agent::{Agent, AgentError};

use crate::{
    commands::{
        Context, Mode,
        args::{self, Validate, ValidateError, validations},
    },
    impl_from_args, operations,
};

#[derive(Args)]
pub struct TransferArgs {
    pub from: Principal,
    pub to: Principal,

    #[arg(long)]
    pub network: Option<args::Network>,
}

impl_from_args!(TransferArgs, from: Principal, to: Principal);
impl_from_args!(TransferArgs, network: Option<args::Network>);

impl Validate for TransferArgs {
    fn validate(&self, mode: &Mode) -> Result<(), ValidateError> {
        // Custom Tests
        for test in [
            //
            // `from` and `to` are the same
            |args, _| {
                let &TransferArgs { from, to, .. } = args;
                (from == to).then_some("`from` and `to` cannot be the same IDs".to_string())
            },
            //
            // dummy case to shush linter
            |_, _| None,
        ] {
            test(self, mode)
                .map(|msg| anyhow::format_err!(msg))
                .map_or(Ok(()), Err)?;
        }

        // General Tests
        for test in [
            validations::a_network_name_is_required_in_project_mode,
            validations::a_network_url_is_required_in_global_mode,
        ] {
            test(self, mode)
                .map(|msg| anyhow::format_err!(msg))
                .map_or(Ok(()), Err)?;
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("failed to make transfer")]
    Transfer(#[from] operations::token::TransferError),

    #[error(transparent)]
    Agent(#[from] AgentError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

pub async fn transfer(ctx: &Context, args: &TransferArgs) -> Result<(), CommandError> {
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

#[cfg(test)]
mod test_args {
    use candid::Principal;

    use crate::commands::{
        args::{Validate, validations::helpers::IntoOptions},
        token::transfer::{TransferArgs, validations},
    };

    #[test]
    fn args_from_and_to_cannot_be_the_same() {
        let tests = [(
            //
            // Args
            validations::helpers::all_networks()
                .into_options()
                .into_iter()
                .map(|network| TransferArgs {
                    from: Principal::anonymous(),
                    to: Principal::anonymous(),
                    network,
                }),
            //
            // Modes
            validations::helpers::all_modes(),
            //
            // Message
            "`from` and `to` cannot be the same IDs",
        )];

        for (args, modes, msg) in tests {
            for v in args {
                for mode in &modes {
                    match (v).validate(mode) {
                        Ok(_) => panic!("expected invalid args"),
                        Err(err) => assert_eq!(err.to_string(), msg),
                    };
                }
            }
        }
    }
}
