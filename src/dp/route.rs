use crate::Stop;
use im::Vector;
use std::alloc::Allocator;

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

impl<A: Allocator> From<Route> for crate::Route<A> {
    fn from(route: Route) -> Self {
        Self::new(route.stops.into_iter().collect())
    }
}
