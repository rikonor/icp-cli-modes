use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use candid::Principal;
use ic_agent::Agent;

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

#[async_trait]
pub trait Transfer: Sync + Send {
    async fn transfer(&self, from: &Principal, to: &Principal) -> Result<(), Error>;
}

pub struct Transmitter;

impl Transmitter {
    pub fn arc(agent: &Agent) -> Arc<dyn Transfer> {
        Arc::new(Transmitter)
    }
}

#[async_trait]
impl Transfer for Transmitter {
    async fn transfer(&self, from: &Principal, to: &Principal) -> Result<(), Error> {
        Ok(())
    }
}
