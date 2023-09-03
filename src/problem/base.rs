use crate::Location;

pub trait BaseProblem {
    fn vehicle_count(&self) -> usize;
    fn stop_count(&self) -> usize;
    fn stop_location(&self, index: usize) -> &Location;
}
