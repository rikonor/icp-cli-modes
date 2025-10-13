use anyhow::{Error, anyhow};
use candid::Principal;
use clap::Args;
use ic_agent::Agent;

use crate::commands::{
    Context, Mode,
    args::{self, Validate, ValidateError},
};

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

#[cfg(test)]
mod tests {
    use candid::Principal;

    use crate::commands::{
        Mode,
        args::{Network, Validate},
        token::transfer::TransferArgs,
    };

    #[test]
    fn args_valid_global() {
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
            (v).validate(&Mode::Global).expect("expected valid args");
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
}
