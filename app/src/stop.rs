use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Stop {
    location: usize,
}

impl Stop {
    pub fn new(location: usize) -> Self {
        Self { location }
    }

    pub fn location(&self) -> usize {
        self.location
    }
}