pub mod canister;
pub mod token;

#[derive(Default)]
pub struct Initializers {
    pub canister: canister::Initializers,
    pub token: token::Initializers,
}
