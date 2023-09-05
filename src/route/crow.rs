use super::Router;
use crate::Location;
use geo::GeodesicDistance;

#[derive(Debug, Default)]
pub struct CrowRouter {}

impl CrowRouter {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Router for &CrowRouter {
    fn route(&self, start: &Location, end: &Location) -> f64 {
        start.as_point().geodesic_distance(end.as_point())
    }
}
