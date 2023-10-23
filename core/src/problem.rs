use crate::{BasicStop, BasicVehicle, Location};

pub trait BasicProblem<V: BasicVehicle, S: BasicStop> {
    fn vehicle_count(&self) -> usize;
    fn vehicle(&self, index: usize) -> &V;

    fn stop_count(&self) -> usize;
    fn stop(&self, index: usize) -> &S;

    fn location_count(&self) -> usize;
    fn location(&self, index: usize) -> &Location;
}

impl<V: BasicVehicle, S: BasicStop, P: BasicProblem<V, S>> BasicProblem<V, S> for &P {
    fn vehicle_count(&self) -> usize {
        (*self).vehicle_count()
    }

    fn vehicle(&self, index: usize) -> &V {
        (*self).vehicle(index)
    }

    fn stop_count(&self) -> usize {
        (*self).stop_count()
    }

    fn stop(&self, index: usize) -> &S {
        (*self).stop(index)
    }

    fn location_count(&self) -> usize {
        (*self).location_count()
    }

    fn location(&self, index: usize) -> &Location {
        (*self).location(index)
    }
}
