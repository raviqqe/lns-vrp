use crate::{Stop, Vehicle};
use alloc::vec::Vec;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Problem {
    vehicles: Vec<Vehicle>,
    stops: Vec<Stop>,
}

impl Problem {
    pub fn new(vehicles: Vec<Vehicle>, stops: Vec<Stop>) -> Self {
        Self { vehicles, stops }
    }

    pub fn vehicles(&self) -> &[Vehicle] {
        &self.vehicles
    }

    pub fn stops(&self) -> &[Stop] {
        &self.stops
    }
}
