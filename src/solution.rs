use crate::Stop;
use alloc::vec::Vec;

// TODO Use persistent data structure.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Solution {
    routes: Vec<Vec<usize>>,
}

impl Solution {
    pub fn new(routes: Vec<Vec<usize>>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &[Vec<usize>] {
        &self.routes
    }

    pub fn add_stop(&self, vehicle_index: usize, stop_index: usize) -> Self {
        let mut route = self.routes[vehicle_index].clone();
        route.push(stop_index);

        let mut routes = self.routes.clone();
        routes[vehicle_index] = route;

        Self { routes }
    }
}
