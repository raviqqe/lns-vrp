use core::BasicStop;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Stop {
    location: usize,
}

impl Stop {
    pub fn new(location: usize) -> Self {
        Self { location }
    }
}

impl BasicStop for Stop {
    fn location(&self) -> usize {
        self.location
    }
}
