use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use candid::Principal;
use ic_agent::Agent;

pub struct Initializers {
    pub start: Box<dyn Fn(&Agent) -> Arc<dyn Start>>,
    pub stop: Box<dyn Fn(&Agent) -> Arc<dyn Stop>>,
}

#[async_trait]
pub trait Start: Sync + Send {
    async fn start(&self, cid: &Principal) -> Result<(), Error>;
}

pub struct Starter;

impl Starter {
    pub fn arc(agent: &Agent) -> Arc<dyn Start> {
        Arc::new(Starter)
    }
}

#[async_trait]
impl Start for Starter {
    async fn start(&self, cid: &Principal) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait Stop: Sync + Send {
    async fn stop(&self, cid: &Principal) -> Result<(), Error>;
}

pub struct Stopper;

impl Stopper {
    pub fn arc(agent: &Agent) -> Arc<dyn Stop> {
        Arc::new(Stopper)
    }
}

#[async_trait]
impl Stop for Stopper {
    async fn stop(&self, cid: &Principal) -> Result<(), Error> {
        Ok(())
    }
}
