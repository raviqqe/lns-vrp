use super::CostCalculator;
use crate::{cost::distance::calculate_route, Stop};

/// Delivery VRP cost calculator.
///
/// All stops are considered as delivery ones.
#[derive(Debug, Default)]
pub struct DeliveryCostCalculator {
    delivery_count: usize,
    missed_delivery_cost: f64,
    // Meters to cost
    distance_unit_cost: f64,
}

impl DeliveryCostCalculator {
    pub fn new(delivery_count: usize, missed_delivery_cost: f64, distance_unit_cost: f64) -> Self {
        Self {
            delivery_count,
            missed_delivery_cost,
            distance_unit_cost,
        }
    }
}

impl CostCalculator for DeliveryCostCalculator {
    fn calculate<'a>(
        &self,
        routes: impl IntoIterator<
            Item = impl IntoIterator<
                Item = &'a Stop,
                IntoIter = impl ExactSizeIterator<Item = &'a Stop>,
            >,
        >,
    ) -> f64 {
        let mut cost = 0.0;
        let mut delivery_count = 0;

        for stops in routes {
            let stops = stops.into_iter();

            delivery_count += stops.len();
            cost += calculate_route(stops);
        }

        cost * self.distance_unit_cost
            + (self.delivery_count - delivery_count) as f64 * self.missed_delivery_cost
    }
}
