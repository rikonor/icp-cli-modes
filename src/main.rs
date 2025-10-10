use std::path::PathBuf;

use anyhow::Error;
use clap::Parser;

use crate::commands::{Command, Context, Mode, canister, token};

mod commands;
mod operations;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let mode = match locate() {
        // Project
        Ok(dir) => Mode::Project(dir),

        // Global
        Err(LocateError::NotFound) => Mode::Global,

        // Failure
        Err(LocateError::Unexpected(err)) => panic!("{err}"),
    };

    let ctx = Context { mode };

    match cli.command {
        Command::Canister(cmd) => match cmd.command {
            canister::Commands::Start(args) => canister::start(&ctx, &args).await?,
            canister::Commands::Stop(args) => canister::stop(&ctx, &args).await?,
        },

        Command::Token(cmd) => match cmd.command {
            token::Commands::Transfer(args) => token::transfer(&ctx, &args).await?,
        },
    }

    Ok(())
}

pub enum LocateError {
    NotFound,
    Unexpected(String),
}

fn locate() -> Result<PathBuf, LocateError> {
    Err(LocateError::NotFound)
}
