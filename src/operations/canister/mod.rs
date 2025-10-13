use std::sync::Arc;

use ic_agent::Agent;

mod start;
pub use start::*;

mod stop;
pub use stop::*;

pub struct Initializers {
    pub start: Box<dyn Fn(&Agent) -> Arc<dyn Start>>,
    pub stop: Box<dyn Fn(&Agent) -> Arc<dyn Stop>>,
}

impl Default for Initializers {
    fn default() -> Self {
        Self {
            start: Box::new(|_| unimplemented!()),
            stop: Box::new(|_| unimplemented!()),
        }
    }
}
