use crate::{Stop, Vehicle};
use alloc::vec::Vec;

use super::BaseProblem;

#[derive(Clone, Debug)]
pub struct SimpleProblem {
    vehicles: Vec<Vehicle>,
    stops: Vec<Stop>,
}

impl SimpleProblem {
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

impl BaseProblem for &SimpleProblem {
    fn vehicle_count(&self) -> usize {
        self.vehicles.len()
    }

    fn stop_count(&self) -> usize {
        self.stops.len()
    }
}
