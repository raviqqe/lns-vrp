use super::solver::Solver;
use crate::{cost::CostCalculator, Problem, Solution};
use ordered_float::OrderedFloat;
use std::collections::BTreeSet;

/// Dynamic programming solver.
///
/// Note that it doesn't use any dynamic programming if you don't provide a cost
/// function that returns infinity.
pub struct DynamicProgrammingSolver<C: CostCalculator> {
    cost_calculator: C,
}

impl<C: CostCalculator> DynamicProgrammingSolver<C> {
    pub fn new(cost_calculator: C) -> Self {
        Self { cost_calculator }
    }
}

impl<C: CostCalculator> Solver for DynamicProgrammingSolver<C> {
    fn solve(&mut self, problem: &Problem) -> Solution {
        // We use a B-tree set instead of a hash one for determinism.
        let mut solutions = BTreeSet::new();

        solutions.insert(Solution::new(
            problem
                .vehicles()
                .iter()
                .map(|_| Default::default())
                .collect(),
        ));

        for stop_index in 0..problem.stops().len() {
            let mut new_solutions = solutions.clone();

            for solution in &solutions {
                for vehicle_index in 0..solution.routes().len() {
                    let solution = solution.add_stop(vehicle_index, stop_index);

                    if self.cost_calculator.calculate(&solution).is_finite() {
                        new_solutions.insert(solution);
                    }
                }
            }

            solutions = new_solutions;
        }

        solutions
            .into_iter()
            .map(|solution| {
                let cost = self.cost_calculator.calculate(&solution);
                (solution, cost)
            })
            .min_by(|(_, one), (_, other)| OrderedFloat(*one).cmp(&OrderedFloat(*other)))
            .expect("at least one solution")
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cost::{DeliveryCostCalculator, DistanceCostCalculator},
        Location, Stop, Vehicle,
    };

    const DISTANCE_COST: f64 = 1.0;
    const MISSED_DELIVERY_COST: f64 = 1e9;
    const QUADRATIC_DISTANCE_COST: f64 = 1e-9;

    fn solve(problem: &Problem) -> Solution {
        let mut distance_cost_calculator = DistanceCostCalculator::new(problem);

        DynamicProgrammingSolver::new(DeliveryCostCalculator::new(
            &mut distance_cost_calculator,
            problem.stops().len(),
            MISSED_DELIVERY_COST,
            DISTANCE_COST,
            QUADRATIC_DISTANCE_COST,
        ))
        .solve(problem)
    }

    #[test]
    fn do_nothing() {
        let problem = Problem::new(vec![Vehicle::new()], vec![]);

        assert_eq!(solve(&problem), Solution::new(vec![vec![]]));
    }

    #[test]
    fn keep_one_stop() {
        let problem = Problem::new(
            vec![Vehicle::new()],
            vec![Stop::new(Location::new(0.0, 0.0))],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![0]]));
    }

    #[test]
    fn keep_two_stops() {
        let problem = Problem::new(
            vec![Vehicle::new()],
            vec![
                Stop::new(Location::new(0.0, 0.0)),
                Stop::new(Location::new(1.0, 0.0)),
            ],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![0, 1]]));
    }

    #[test]
    fn keep_three_stops() {
        let problem = Problem::new(
            vec![Vehicle::new()],
            vec![
                Stop::new(Location::new(0.0, 0.0)),
                Stop::new(Location::new(1.0, 0.0)),
                Stop::new(Location::new(2.0, 0.0)),
            ],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![0, 1, 2]]));
    }

    #[test]
    fn even_workload() {
        let problem = Problem::new(
            vec![Vehicle::new(), Vehicle::new()],
            vec![
                Stop::new(Location::new(0.0, 0.0)),
                Stop::new(Location::new(1.0, 0.0)),
                Stop::new(Location::new(2.0, 0.0)),
            ],
        );

        assert!(solve(&problem).routes().iter().all(|stops| stops.len() < 3));
    }
}
