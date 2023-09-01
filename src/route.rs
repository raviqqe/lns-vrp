use crate::{List, Stop};

#[derive(Clone, Debug)]
pub struct Route {
    stops: List<Stop>,
}

impl Route {
    pub fn new(stops: Vec<Stop>) -> Self {
        Self { stops }
    }

    pub fn stops(&self) -> impl Iterator<Item = &Stop> {
        self.stops.iter()
    }
}
