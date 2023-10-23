use crate::{Stop, Vehicle};
use alloc::vec::Vec;
use core::{BasicProblem, BasicStop, BasicVehicle, Location};
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

impl core::BasicProblem<Vehicle, Stop> for SimpleProblem {
    fn vehicle_count(&self) -> usize {
        self.vehicles.len()
    }

    fn vehicle(&self, index: usize) -> &Vehicle {
        &self.vehicles[index]
    }

    fn stop_count(&self) -> usize {
        self.stops.len()
    }

    fn stop(&self, index: usize) -> &Stop {
        &self.stops[index]
    }

    fn location_count(&self) -> usize {
        self.locations.len()
    }

    fn location(&self, index: usize) -> &Location {
        &self.locations[index]
    }
}
