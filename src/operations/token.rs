use anyhow::Error;
use async_trait::async_trait;
use candid::Principal;

#[async_trait]

pub trait Transfer: Sync + Send {
    async fn transfer(from: &Principal, to: &Principal) -> Result<(), Error>;
}

pub async fn transfer(from: &Principal, to: &Principal) -> Result<(), Error> {
    Ok(())
}
