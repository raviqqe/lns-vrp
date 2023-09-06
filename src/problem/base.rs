use crate::Location;

pub trait BaseProblem {
    fn vehicle_count(&self) -> usize;
    fn vehicle_start_location(&self, index: usize) -> usize;
    fn vehicle_end_location(&self, index: usize) -> usize;

    fn stop_count(&self) -> usize;
    fn stop_location(&self, index: usize) -> usize;

    fn location_count(&self) -> usize;
    fn location(&self, index: usize) -> &Location;
}
