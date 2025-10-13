use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use candid::Principal;
use ic_agent::Agent;

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
