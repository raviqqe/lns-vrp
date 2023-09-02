use crate::Stop;
use geo::GeodesicDistance;

pub fn calculate_route<'a>(stops: impl IntoIterator<Item = &'a Stop>) -> f64 {
    let mut cost = 0.0;
    let mut stops = stops.into_iter();

    if let Some(mut previous) = stops.next() {
        for stop in stops {
            cost += calculate_segment(previous, stop);
            previous = stop;
        }
    }

    cost
}

pub fn calculate_segment(one: &Stop, other: &Stop) -> f64 {
    one.location()
        .as_point()
        .geodesic_distance(other.location().as_point())
}
