use crate::location::Location;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Stop {
    location: Location,
}

impl Stop {
    pub fn new(location: Location) -> Self {
        Self { location }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}
