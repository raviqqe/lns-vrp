use super::solver::Solver;
use crate::{cost::CostCalculator, hash_map::HashMap, problem::BaseProblem, Solution};
use bumpalo::Bump;
use ordered_float::OrderedFloat;

pub struct NearestNeighbourSolver<C: CostCalculator> {
    cost_calculator: C,
}

impl<C: CostCalculator> NearestNeighbourSolver<C> {
    pub fn new(cost_calculator: C) -> Self {
        Self { cost_calculator }
    }
}

impl<C: CostCalculator> Solver for NearestNeighbourSolver<C> {
    fn solve(&mut self, problem: impl BaseProblem) -> Solution {
        let solution = Solution::new((0..problem.vehicle_count()).map(|_| Default::default()));

        for stop_index in 0..problem.stop_count() {
            foo
        }

        solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cost::{DeliveryCostCalculator, DistanceCostCalculator},
        Location, SimpleProblem, Stop, Vehicle,
    };

    const DISTANCE_COST: f64 = 1.0;
    const MISSED_DELIVERY_COST: f64 = 1e9;

    fn solve(problem: &SimpleProblem) -> Solution {
        NearestNeighbourSolver::new(DeliveryCostCalculator::new(
            DistanceCostCalculator::new(problem),
            problem.stops().len(),
            MISSED_DELIVERY_COST,
            DISTANCE_COST,
        ))
        .solve(problem)
    }

    #[test]
    fn do_nothing() {
        let problem = SimpleProblem::new(vec![Vehicle::new()], vec![]);

        assert_eq!(solve(&problem), Solution::new(vec![vec![]]));
    }

    #[test]
    fn keep_one_stop() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new()],
            vec![Stop::new(Location::new(0.0, 0.0))],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![0]]));
    }

    #[test]
    fn keep_two_stops() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new()],
            vec![
                Stop::new(Location::new(0.0, 0.0)),
                Stop::new(Location::new(1.0, 0.0)),
            ],
        );

        assert_eq!(solve(&problem).routes()[0].len(), 2);
    }

    #[test]
    fn keep_three_stops() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new()],
            vec![
                Stop::new(Location::new(0.0, 0.0)),
                Stop::new(Location::new(1.0, 0.0)),
                Stop::new(Location::new(2.0, 0.0)),
            ],
        );

        assert_eq!(solve(&problem).routes()[0][1], 1);
    }

    #[test]
    fn optimize_stop_order() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new()],
            vec![
                Stop::new(Location::new(0.0, 0.0)),
                Stop::new(Location::new(2.0, 0.0)),
                Stop::new(Location::new(1.0, 0.0)),
            ],
        );

        assert_eq!(solve(&problem).routes()[0][1], 2);
    }

    #[test]
    fn even_workload() {
        let problem = SimpleProblem::new(
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
