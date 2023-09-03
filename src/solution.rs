use crate::Stop;
use alloc::vec::Vec;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Solution {
    vehicles: Vec<Vehicle>,
    stops: Vec<Stop>,
}

impl Problem {
    pub fn new(vehicles: Vec<Vehicle>, stops: Vec<Stop>) -> Self {
        Self { vehicles, stops }
    }

    pub fn vehilcles(&self) -> Foo {
        &self.vehicles
    }

    pub fn stops(&self) -> &[Stop] {
        &self.stops
    }
}
