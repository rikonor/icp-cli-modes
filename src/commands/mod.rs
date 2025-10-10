use std::path::PathBuf;

use clap::Subcommand;

pub mod args;
pub mod canister;
pub mod token;

#[derive(Subcommand)]
pub enum Command {
    // Canister
    Canister(canister::Command),

    // Token
    Token(token::Command),
}

pub struct Context {
    pub mode: Mode,
}

pub enum Mode {
    Project(PathBuf),
    Global,
}

#[derive(Debug, thiserror::Error)]
pub enum ValidateError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

trait Validate {
    fn validate(&self, mode: &Mode) -> Result<(), ValidateError>;
}

pub trait BoolSliceExt {
    fn all(&self) -> bool;
}

impl BoolSliceExt for [bool] {
    fn all(&self) -> bool {
        !self.contains(&false)
    }
}
