#[derive(Debug, Eq, Hash, PartialEq)]
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
