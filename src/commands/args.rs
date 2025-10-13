use candid::Principal;

use crate::commands::Mode;

#[derive(Clone, Debug, PartialEq)]
pub enum Canister {
    Name(String),
    Principal(Principal),
}

impl From<&str> for Canister {
    fn from(v: &str) -> Self {
        if let Ok(p) = Principal::from_text(v) {
            return Self::Principal(p);
        }

        Self::Name(v.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Network {
    Name(String),
    Url(String),
}

impl From<&str> for Network {
    fn from(v: &str) -> Self {
        if v.starts_with("http://") || v.starts_with("https://") {
            return Self::Url(v.to_string());
        }

        Self::Name(v.to_string())
    }
}

// TODO? Alias arg for transfers
// can maintain mapping of principal aliases
// e.g to associate a principal with a person, or with a specific canister

#[derive(Debug, thiserror::Error)]
pub enum ValidateError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

pub trait Validate {
    fn validate(&self, mode: &Mode) -> Result<(), ValidateError>;
}

#[cfg(test)]
mod tests {
    use candid::Principal;

    use crate::commands::args::{Canister, Network};

    #[test]
    fn canister_by_name() {
        assert_eq!(
            Canister::from("my-canister"),
            Canister::Name("my-canister".to_string()),
        );
    }

    #[test]
    fn canister_by_principal() {
        let cid = "ntyui-iatoh-pfi3f-27wnk-vgdqt-mq3cl-ld7jh-743kl-sde6i-tbm7g-tqe";

        assert_eq!(
            Canister::from(cid),
            Canister::Principal(Principal::from_text(cid).expect("failed to parse principal")),
        );
    }

    #[test]
    fn network_by_name() {
        assert_eq!(
            Network::from("my-network"),
            Network::Name("my-network".to_string()),
        );
    }

    #[test]
    fn network_by_url_http() {
        let url = "http://www.example.com";

        assert_eq!(
            Network::from(url),
            Network::Url("http://www.example.com".to_string()),
        );
    }
}
