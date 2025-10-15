use std::sync::Arc;

use async_trait::async_trait;
use candid::Principal;
use ic_agent::Agent;

#[derive(Debug, thiserror::Error)]
pub enum TransferError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[async_trait]
pub trait Transfer: Sync + Send {
    async fn transfer(&self, from: &Principal, to: &Principal) -> Result<(), TransferError>;
}

pub struct Transmitter;

impl Transmitter {
    pub fn arc(agent: &Agent) -> Arc<dyn Transfer> {
        Arc::new(Transmitter)
    }
}

#[async_trait]
impl Transfer for Transmitter {
    async fn transfer(&self, from: &Principal, to: &Principal) -> Result<(), TransferError> {
        Ok(())
    }
}
