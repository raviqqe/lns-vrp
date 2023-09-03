use crate::Route;
use alloc::vec::Vec;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Problem {
    vehicles: Vec<Vehicle>,
    stops: Vec<Stop>,
}

impl Problem {
    pub fn new(routes: Vec<Vehicle>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> impl Iterator<Item = &Route> {
        self.routes.iter()
    }
}
