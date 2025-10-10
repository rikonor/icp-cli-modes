use candid::Principal;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[cfg(test)]
mod tests {
    // TODO: tests for the arg parsing
}
