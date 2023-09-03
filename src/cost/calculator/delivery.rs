use super::CostCalculator;
use crate::{cost::distance::calculate_route, Problem, Stop};

/// Delivery VRP cost calculator.
///
/// All stops are considered as delivery ones.
#[derive(Debug)]
pub struct DeliveryCostCalculator<'a> {
    problem: &'a Problem,
    delivery_count: usize,
    missed_delivery_cost: f64,
    distance_cost: f64,
    quadratic_distance_cost: f64,
}

impl<'a> DeliveryCostCalculator<'a> {
    pub fn new(
        problem: &'a Problem,
        delivery_count: usize,
        missed_delivery_cost: f64,
        distance_cost: f64,
        quadratic_distance_cost: f64,
    ) -> Self {
        Self {
            problem,
            delivery_count,
            missed_delivery_cost,
            distance_cost,
            quadratic_distance_cost,
        }
    }

    fn calculate_distance_cost(&self, solution: &Solution) -> f64 {
        solution
            .routes()
            .iter()
            .map(|stop_ids| {
                self.calculate_route_distance_cost(
                    stop_ids.iter().map(|index| &self.problem.stops()[index]),
                )
            })
            .sum::<f64>()
            * self.distance_cost
    }

    fn calculate_delivery_cost(&self, solution: &Solution) -> f64 {
        (self.delivery_count - solution.routes().iter().map(|stops| stops.len()).sum()) as f64
            * self.missed_delivery_cost
    }

    fn calculate_route_distance_cost<'a>(&self, stop_indexes: &[usize]) -> f64 {
        let route_cost = calculate_route(stops);
        route_cost + route_cost.powi(2) * self.quadratic_distance_cost
    }
}

impl CostCalculator for DeliveryCostCalculator {
    fn calculate<'a>(&self, solution: &Solution) -> f64 {
        self.calculate_distance_cost(solution)
            + (self.delivery_count - solution.routes().iter().map(|stops| stops.len()).sum()) as f64
                * self.missed_delivery_cost
    }

    fn calculate_lower_bound<'a>(&self, solution: &Solution) -> f64 {
        self.calculate_distance_cost(solution)
    }
}

// TODO Add tests if a cost calculator trait is stable.
