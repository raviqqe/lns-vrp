use crate::stop::Stop;

#[derive(Clone, Debug)]
pub struct Route {
    stops: Vec<Stop>,
}

impl Route {
    pub fn new(stops: Vec<Stop>) -> Self {
        Self { stops }
    }

    pub fn stops(&self) -> impl Iterator<Item = &Stop> {
        self.stops.iter()
    }
}
