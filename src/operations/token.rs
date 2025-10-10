use anyhow::Error;
use candid::Principal;

pub async fn transfer(from: &Principal, to: &Principal) -> Result<(), Error> {
    Ok(())
}
