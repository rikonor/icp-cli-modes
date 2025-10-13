use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use candid::Principal;
use ic_agent::Agent;
use mockall::automock;

#[automock]
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
