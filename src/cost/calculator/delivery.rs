use super::CostCalculator;
use crate::{cost::distance::calculate_route, Stop};

/// Delivery VRP cost calculator.
///
/// All stops are considered as delivery ones.
#[derive(Debug, Default)]
pub struct DeliveryCostCalculator {
    delivery_count: usize,
    missed_delivery_cost: f64,
    distance_cost: f64,
    quadratic_distance_cost: f64,
}

impl DeliveryCostCalculator {
    pub fn new(
        delivery_count: usize,
        missed_delivery_cost: f64,
        distance_cost: f64,
        quadratic_distance_cost: f64,
    ) -> Self {
        Self {
            delivery_count,
            missed_delivery_cost,
            distance_cost,
            quadratic_distance_cost,
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

            let route_cost = calculate_route(stops);
            cost += route_cost + route_cost.powi(2) * self.quadratic_distance_cost;
        }

        cost * self.distance_cost
            + (self.delivery_count - delivery_count) as f64 * self.missed_delivery_cost
    }
}

// TODO Add tests if a cost calculator trait is stable.
