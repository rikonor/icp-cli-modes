use std::sync::Arc;

use async_trait::async_trait;
use candid::Principal;
use ic_agent::Agent;
use mockall::automock;

#[derive(Debug, thiserror::Error)]
pub enum StopError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[automock]
#[async_trait]
pub trait Stop: Sync + Send {
    async fn stop(&self, cid: &Principal) -> Result<(), StopError>;
}

pub struct Stopper;

impl Stopper {
    pub fn arc(agent: &Agent) -> Arc<dyn Stop> {
        Arc::new(Stopper)
    }
}

#[async_trait]
impl Stop for Stopper {
    async fn stop(&self, cid: &Principal) -> Result<(), StopError> {
        Ok(())
    }
}
