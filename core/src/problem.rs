use crate::{BasicStop, BasicVehicle, Location};

pub trait BasicProblem<V: BasicVehicle, S: BasicStop> {
    fn vehicle_count(&self) -> usize;
    fn vehicle(&self, index: usize) -> &V;

    fn stop_count(&self) -> usize;
    fn stop(&self, index: usize) -> &S;

    fn location_count(&self) -> usize;
    fn location(&self, index: usize) -> &Location;
}
