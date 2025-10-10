pub mod canister;
pub mod token;

pub struct Initializers {
    pub canister: canister::Initializers,
    pub token: token::Initializers,
}
