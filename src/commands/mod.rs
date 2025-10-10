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
