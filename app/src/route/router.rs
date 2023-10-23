use core::Location;

pub trait Router {
    fn route(&self, start: &Location, end: &Location) -> f64;
}
