use std::path::PathBuf;

use clap::Subcommand;

use crate::operations::Initializers;

pub mod args;
pub mod build;
pub mod canister;
pub mod macros;
pub mod token;

#[derive(Subcommand)]
pub enum Command {
    // Build
    Build(build::Arguments),

    // Canister
    Canister(canister::Command),

    // Token
    Token(token::Command),
}

pub struct Context {
    pub mode: Mode,
    pub ops: Initializers,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Project(PathBuf),
    Global,
}
