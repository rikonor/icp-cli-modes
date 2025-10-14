use anyhow::{Error, anyhow};
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

#[cfg(test)]
mod tests {
    use candid::Principal;

    use crate::commands::{
        Mode,
        args::{Network, Validate},
        token::transfer::TransferArgs,
    };

    #[test]
    fn args_valid() {
        let args = [
            TransferArgs {
                from: Principal::anonymous(),
                to: Principal::anonymous(),
                network: None,
            },
            TransferArgs {
                from: Principal::anonymous(),
                to: Principal::anonymous(),
                network: Some(Network::Url("http://www.example.com".to_string())),
            },
        ];

        for v in args {
            // Mode (Global)
            (v).validate(&Mode::Global).expect("expected valid args");

            // Mode (Project)
            (v).validate(&Mode::Project("dir".into()))
                .expect("expected valid args");
        }
    }

    #[test]
    fn args_invalid_global() {
        let args = [(
            TransferArgs {
                from: Principal::anonymous(),
                to: Principal::anonymous(),
                network: Some(Network::Name("my-network".to_string())),
            },
            "please provide a network url",
        )];

        for (v, msg) in args {
            match (v).validate(&Mode::Global) {
                Ok(_) => panic!("expected invalid args"),
                Err(err) => assert_eq!(err.to_string(), msg),
            };
        }
    }

    #[test]
    fn args_invalid_project() {
        let args = [(
            TransferArgs {
                from: Principal::anonymous(),
                to: Principal::anonymous(),
                network: Some(Network::Url("http://www.example.com".to_string())),
            },
            "please provide a network name",
        )];

        for (v, msg) in args {
            match (v).validate(&Mode::Project("project-dir".into())) {
                Ok(_) => panic!("expected invalid args"),
                Err(err) => assert_eq!(err.to_string(), msg),
            };
        }
    }
}
