use std::sync::Arc;

use ic_agent::Agent;

mod transfer;
pub use transfer::*;

pub struct Initializers {
    pub transfer: Box<dyn Fn(&Agent) -> Arc<dyn Transfer>>,
}

impl Default for Initializers {
    fn default() -> Self {
        Self {
            transfer: Box::new(|_| unimplemented!()),
        }
    }
}
