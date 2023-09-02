use crate::Stop;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Route {
    stops: Vec<Stop>,
}

impl Route {
    pub fn new(stops: Vec<Stop>) -> Self {
        Self { stops }
    }

    pub fn stops(&self) -> impl ExactSizeIterator<Item = &Stop> {
        self.stops.iter()
    }
}
