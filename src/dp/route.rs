use crate::Stop;
use im::Vector;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Route {
    stops: Vector<Stop>,
}

impl Route {
    pub fn new(stops: &[Stop]) -> Self {
        Self {
            stops: Vector::from_iter(stops.iter().cloned()),
        }
    }

    pub fn stops(&self) -> &Vector<Stop> {
        &self.stops
    }
}
