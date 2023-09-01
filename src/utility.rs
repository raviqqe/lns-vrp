use crate::{Location, Stop};

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
    calculate_distance_cost(one.location(), other.location())
}

pub fn calculate_distance_cost(one: &Location, other: &Location) -> f64 {
    ((one.latitude() - other.latitude()).powi(2) + (one.latitude() - other.latitude()).powi(2))
        .sqrt()
}
