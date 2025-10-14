use anyhow::Error;
use clap::Args;

use crate::commands::Context;

#[derive(Args)]
pub struct Arguments {
    name: Option<String>,
}

pub async fn build(ctx: &Context, args: &Arguments) -> Result<(), Error> {
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

    Ok(())
}
