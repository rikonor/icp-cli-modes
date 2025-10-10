use std::path::PathBuf;

use clap::Subcommand;

use crate::operations::Initializers;

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
    pub ops: Initializers,
}

pub enum Mode {
    Project(PathBuf),
    Global,
}
