use super::CostCalculator;
use crate::{cost::distance::DistanceCostCalculator, problem::BaseProblem, Solution};

#[derive(Debug)]
pub struct PickupAndDropOffCostCalculator<P: BaseProblem> {
    distance_cost_calculator: DistanceCostCalculator<P>,
    delivery_count: usize,
    missed_delivery_cost: f64,
    distance_cost: f64,
    quadratic_distance_cost: f64,
}

impl<P: BaseProblem> PickupAndDropOffCostCalculator<P> {
    pub fn new(
        distance_cost_calculator: DistanceCostCalculator<P>,
        delivery_count: usize,
        missed_delivery_cost: f64,
        distance_cost: f64,
        quadratic_distance_cost: f64,
    ) -> Self {
        Self {
            distance_cost_calculator,
            delivery_count,
            missed_delivery_cost,
            distance_cost,
            quadratic_distance_cost,
        }
    }

    fn calculate_distance_cost(&mut self, solution: &Solution) -> f64 {
        solution
            .routes()
            .iter()
            .map(|stop_indexes| {
                let cost = self.distance_cost_calculator.calculate_route(stop_indexes);

                cost * self.distance_cost + cost.powi(2) * self.quadratic_distance_cost
            })
            .sum()
    }

    fn calculate_delivery_cost(&self, solution: &Solution) -> f64 {
        (self.delivery_count
            - solution
                .routes()
                .iter()
                .map(|stops| stops.len())
                .sum::<usize>()) as f64
            * self.missed_delivery_cost
    }
}

impl<P: BaseProblem> CostCalculator for PickupAndDropOffCostCalculator<P> {
    fn calculate(&mut self, solution: &Solution) -> f64 {
        self.calculate_distance_cost(solution) + self.calculate_delivery_cost(solution)
    }

    fn calculate_lower_bound(&mut self, solution: &Solution) -> f64 {
        self.calculate_distance_cost(solution)
    }
}

// TODO Add tests if a cost calculator trait is stable.
