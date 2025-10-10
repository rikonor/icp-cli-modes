use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use candid::Principal;
use ic_agent::Agent;

#[async_trait]

pub trait Transfer: Sync + Send {
    async fn transfer(&self, from: &Principal, to: &Principal) -> Result<(), Error>;
}

pub struct Initializers {
    pub transfer: Box<dyn Fn(&Agent) -> Arc<dyn Transfer>>,
}
