use crate::Location;

#[derive(Debug, Default, Eq, Hash, PartialEq)]
pub struct Vehicle {
    start_location: Location,
    end_location: Location,
}

impl Vehicle {
    pub fn new(start_location: Location, end_location: Location) -> Self {
        Self {
            start_location,
            end_location,
        }
    }

    pub fn start_location(&self) -> &Location {
        &self.start_location
    }

    pub fn end_location(&self) -> &Location {
        &self.end_location
    }
}
