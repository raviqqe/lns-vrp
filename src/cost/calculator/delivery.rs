use super::CostCalculator;
use crate::Stop;
use geo::GeodesicDistance;

/// Delivery VRP cost calculator.
///
/// All stops are considered as delivery ones.
#[derive(Debug, Default)]
pub struct DeliveryCostCalculator {
    delivery_count: usize,
    missed_delivery_cost: f64,
}

impl DeliveryCostCalculator {
    pub fn new(delivery_count: usize, missed_delivery_cost: f64) -> Self {
        Self {
            delivery_count,
            missed_delivery_cost,
        }
    }

    fn calculate_route<'a>(&self, stops: impl IntoIterator<Item = &'a Stop>) -> f64 {
        let mut cost = 0.0;
        let mut stops = stops.into_iter();

        if let Some(mut previous) = stops.next() {
            for stop in stops {
                cost += self.calculate_segment(previous, stop);
                previous = stop;
            }
        }

        cost
    }

    fn calculate_segment(&self, one: &Stop, other: &Stop) -> f64 {
        one.location()
            .as_point()
            .geodesic_distance(other.location().as_point())
    }
}

impl CostCalculator for DeliveryCostCalculator {
    fn calculate<'a>(
        &self,
        routes: impl IntoIterator<Item = impl IntoIterator<Item = &'a Stop>>,
    ) -> f64 {
        routes
            .into_iter()
            .map(|route| self.calculate_route(route))
            .sum()
            + (self.delivery_count - routes.into_iter().flat_map(|stops| stops.len()).sum()) as f64
                * self.missed_delivery_cost
    }
}
