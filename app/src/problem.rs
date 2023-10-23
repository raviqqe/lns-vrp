use crate::{Location, Stop, Vehicle};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleProblem {
    vehicles: Vec<Vehicle>,
    stops: Vec<Stop>,
    locations: Vec<Location>,
}

impl SimpleProblem {
    pub fn new(vehicles: Vec<Vehicle>, stops: Vec<Stop>, locations: Vec<Location>) -> Self {
        Self {
            vehicles,
            stops,
            locations,
        }
    }

    pub fn vehicles(&self) -> &[Vehicle] {
        &self.vehicles
    }

    pub fn stops(&self) -> &[Stop] {
        &self.stops
    }

    pub fn to_json(&self) -> Result<serde_json::value::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    pub fn from_json(value: serde_json::value::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

impl core::BasicProblem for &SimpleProblem {
    fn vehicle_count(&self) -> usize {
        self.vehicles.len()
    }

    fn vehicle_start_location(&self, index: usize) -> usize {
        self.vehicles[index].start_location()
    }

    fn vehicle_end_location(&self, index: usize) -> usize {
        self.vehicles[index].end_location()
    }

    fn stop_count(&self) -> usize {
        self.stops.len()
    }

    fn stop_location(&self, index: usize) -> usize {
        self.stops[index].location()
    }

    fn location_count(&self) -> usize {
        self.locations.len()
    }

    fn location(&self, index: usize) -> &Location {
        &self.locations[index]
    }
}
