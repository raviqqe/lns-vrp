use super::solver::Solver;
use crate::{cost::CostCalculator, Problem, Solution};
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

pub struct BranchAndBoundSolver<C: CostCalculator> {
    cost_calculator: C,
}

impl<C: CostCalculator> BranchAndBoundSolver<C> {
    pub fn new(cost_calculator: C) -> Self {
        Self { cost_calculator }
    }
}

impl<C: CostCalculator> Solver for BranchAndBoundSolver<C> {
    fn solve(&mut self, problem: &Problem) -> Solution {
        let mut solutions = BTreeMap::<Solution, f64>::new();
        let routes = Solution::new(
            problem
                .vehicles()
                .iter()
                .map(|_| Default::default())
                .collect(),
        );

        let cost = self.cost_calculator.calculate(&routes);
        solutions.insert(routes, cost);

        for stop_index in 0..problem.stops().len() {
            let mut new_states = solutions.clone();

            for (solution, upper_bound) in &solutions {
                for vehicle_index in 0..solution.routes().len() {
                    let solution = solution.add_stop(vehicle_index, stop_index);
                    let lower_bound = self.cost_calculator.calculate_lower_bound(&solution);

                    if lower_bound < *upper_bound {
                        let cost = self.cost_calculator.calculate(&solution);
                        new_states.insert(solution, cost);
                    }
                }
            }

            solutions = new_states;
        }

        solutions
            .into_iter()
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
        BranchAndBoundSolver::new(DeliveryCostCalculator::new(
            DistanceCostCalculator::new(problem),
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
