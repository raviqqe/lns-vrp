use core::BasicVehicle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
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
}

impl BasicVehicle for Vehicle {
    fn start_location(&self) -> usize {
        self.start_location
    }

    fn end_location(&self) -> usize {
        self.end_location
    }
}
