use anyhow::Error;
use async_trait::async_trait;
use candid::Principal;

#[async_trait]
pub trait Start: Sync + Send {
    async fn start(cid: &Principal) -> Result<(), Error>;
}

#[async_trait]
pub trait Stop: Sync + Send {
    async fn stop(cid: &Principal) -> Result<(), Error>;
}

pub async fn start(cid: &Principal) -> Result<(), Error> {
    Ok(())
}

pub async fn stop(cid: &Principal) -> Result<(), Error> {
    Ok(())
}
