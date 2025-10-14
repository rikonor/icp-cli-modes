use indoc::formatdoc;

use crate::commands::{
    Mode,
    args::{Canister, Network},
};

#[derive(Debug, thiserror::Error)]
pub enum ValidateError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

pub trait Validate {
    fn validate(&self, mode: &Mode) -> Result<(), ValidateError>;
}

pub fn a_canister_id_is_required_in_global_mode<'a>(
    canister: impl Into<(&'a Canister,)>,
    m: &Mode,
) -> Option<String> {
    let (canister,) = canister.into();

    (matches!(m, Mode::Global) && matches!(canister, Canister::Principal(_))).then_some(
        formatdoc! {"
                Please provide a canister principal in global mode.
        "},
    )
}

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
