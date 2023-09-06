use crate::Location;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Vehicle {
    start_location: usize,
    end_location: usize,
}

impl Vehicle {
    pub fn new(start_location: usize, end_location: usize) -> Self {
        Self {
            start_location,
            end_location,
        }
    }

    pub fn start_location(&self) -> usize {
        self.start_location
    }

    pub fn end_location(&self) -> usize {
        self.end_location
    }
}
