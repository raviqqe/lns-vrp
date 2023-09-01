use crate::{Location, Stop};
use geo::GeodesicDistance;

pub fn calculate_route_cost<'a>(stops: impl IntoIterator<Item = &'a Stop>) -> f64 {
    let mut cost = 0.0;
    let mut stops = stops.into_iter();

    if let Some(mut previous) = stops.next() {
        for stop in stops {
            cost += calculate_route_segment_cost(previous, stop);
            previous = stop;
        }
    }

    cost
}

pub fn calculate_route_segment_cost(one: &Stop, other: &Stop) -> f64 {
    calculate_distance(one.location(), other.location())
}

pub fn calculate_distance(one: &Location, other: &Location) -> f64 {
    one.as_point().geodesic_distance(other.as_point())
}
