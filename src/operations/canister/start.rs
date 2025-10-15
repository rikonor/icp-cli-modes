use std::sync::Arc;

use async_trait::async_trait;
use candid::Principal;
use ic_agent::Agent;
use mockall::automock;

#[derive(Debug, thiserror::Error)]
pub enum StartError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[automock]
#[async_trait]
pub trait Start: Sync + Send {
    async fn start(&self, cid: &Principal) -> Result<(), StartError>;
}

pub struct Starter;

impl Starter {
    pub fn arc(agent: &Agent) -> Arc<dyn Start> {
        Arc::new(Starter)
    }
}

#[async_trait]
impl Start for Starter {
    async fn start(&self, cid: &Principal) -> Result<(), StartError> {
        Ok(())
    }
}
