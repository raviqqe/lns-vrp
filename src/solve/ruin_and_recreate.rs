use super::solver::Solver;
use crate::{cost::CostCalculator, problem::BaseProblem, Solution};

pub struct RuinAndRecreateSolver<C: CostCalculator> {
    cost_calculator: C,
    iteration_count: usize,
}

impl<C: CostCalculator> RuinAndRecreateSolver<C> {
    pub fn new(cost_calculator: C, iteration_count: usize) -> Self {
        Self {
            cost_calculator,
            iteration_count,
        }
    }
}

impl<C: CostCalculator> Solver for RuinAndRecreateSolver<C> {
    fn solve(&mut self, problem: impl BaseProblem) -> Solution {
        if problem.vehicle_count() == 0 {
            return Solution::new(vec![]);
        }

        let mut solution = Solution::new({
            let mut routes = Vec::with_capacity(problem.vehicle_count());
            routes.push((0..problem.stop_count()).collect());
            routes.extend((1..problem.vehicle_count()).map(|_| Default::default()));
            routes
        });
        let mut cost = self.cost_calculator.calculate(&solution);

        for _ in 0..self.iteration_count {
            let region = self.choose_region(&solution);

            if self.cost_calculator.calculate(&new_solution) < cost {
                solution = new_solution;
            }
        }

        solution
    }

    fn choose_region(&self, solution: &Solution) -> foo {
        if rand::random::<bool>() {
            todo!()
        } else {
            todo!()
        }
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
        RuinAndRecreateSolver::new(DeliveryCostCalculator::new(
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

        assert_eq!(solve(&problem), Solution::new(vec![vec![0, 1]]));
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

        assert_eq!(solve(&problem), Solution::new(vec![vec![0, 1, 2]]));
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
