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

pub mod validations {
    use indoc::formatdoc;

    use crate::commands::{Mode, args::Network};

    pub fn network_or_environment_not_both<'a>(
        network_environment: impl Into<(&'a Option<Network>, &'a Option<String>)>,
        m: &Mode,
    ) -> Option<String> {
        let (network, environment) = network_environment.into();

        (matches!(m, _) && network.is_some() && environment.is_some()).then_some(formatdoc! {"
        Please provide either a network or an environment, but not both.
    "})
    }

    pub fn environments_are_not_available_in_a_global_mode<'a>(
        environment: impl Into<(&'a Option<String>,)>,
        m: &Mode,
    ) -> Option<String> {
        let (environment,) = environment.into();

        (matches!(m, Mode::Global) && environment.is_some()).then_some(formatdoc! {"
        Environments are not available in global mode.
    "})
    }

    pub fn a_network_url_is_required_in_global_mode<'a>(
        network: impl Into<(&'a Option<Network>,)>,
        m: &Mode,
    ) -> Option<String> {
        let (network,) = network.into();

        (matches!(m, Mode::Global) && !matches!(network, Some(Network::Url(_)))).then_some(
            formatdoc! {"
            A network `url` is required in global mode.
        "},
        )
    }

    pub fn a_network_name_is_required_in_project_mode<'a>(
        network: impl Into<(&'a Option<Network>,)>,
        m: &Mode,
    ) -> Option<String> {
        let (network,) = network.into();

        (matches!(m, Mode::Project(_)) && !matches!(network, Some(Network::Name(_)))).then_some(
            formatdoc! {"
            A network `name` is required in project mode.
        "},
        )
    }
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
