use crate::Location;

pub trait BaseProblem {
    fn vehicle_count(&self) -> usize;
    fn vehicle_start_location(&self, index: usize) -> &Location;
    fn vehicle_end_location(&self, index: usize) -> &Location;
    fn stop_count(&self) -> usize;
    fn stop_location(&self, index: usize) -> &Location;
}
