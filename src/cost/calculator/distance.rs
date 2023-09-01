use super::CostCalculator;
use crate::Stop;

pub struct DistanceCostCalculator {}

impl DistanceCostCalculator {
    fn calculate_route_cost<'a>(&self, stops: impl IntoIterator<Item = &'a Stop>) -> f64 {
        let mut cost = 0.0;
        let mut stops = stops.into_iter();

        if let Some(mut previous) = stops.next() {
            for stop in stops {
                cost += self.calculate_route_segment_cost(previous, stop);
                previous = stop;
            }
        }

        cost
    }

    fn calculate_route_segment_cost(&self, one: &Stop, other: &Stop) -> f64 {
        one.location()
            .as_point()
            .geodesic_distance(other.location().as_point())
    }
}

impl CostCalculator for DistanceCostCalculator {
    fn calculate<'a>(
        &self,
        routes: impl IntoIterator<Item = impl IntoIterator<Item = &'a Stop>>,
    ) -> f64 {
        0.0
    }
}
