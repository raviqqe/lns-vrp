use super::solver::Solver;
use crate::{cost::CostCalculator, hash_map::HashMap, problem::BaseProblem, Solution};
use bumpalo::Bump;
use ordered_float::OrderedFloat;

pub struct BranchAndBoundSolver<C: CostCalculator> {
    cost_calculator: C,
}

impl<C: CostCalculator> BranchAndBoundSolver<C> {
    pub fn new(cost_calculator: C) -> Self {
        Self { cost_calculator }
    }
}

impl<C: CostCalculator> Solver for BranchAndBoundSolver<C> {
    fn solve(&mut self, problem: impl BaseProblem) -> Solution {
        let allocator = Bump::new();
        let mut solutions = HashMap::default();
        let solution =
            Solution::new({ (0..problem.vehicle_count()).map(|_| PVec::new()).collect() });
        let cost = self.cost_calculator.calculate(&solution);
        solutions.insert(solution, cost);
        let mut new_solutions = vec![];

        for _ in 0..problem.stop_count() {
            new_solutions.clear();

            for (solution, upper_bound) in &solutions {
                for stop_index in 0..problem.stop_count() {
                    if solution.has_stop(stop_index) {
                        continue;
                    }

                    for vehicle_index in 0..solution.routes().len() {
                        let solution = solution.add_stop(vehicle_index, stop_index);
                        let lower_bound = self.cost_calculator.calculate_lower_bound(&solution);

                        if lower_bound < *upper_bound {
                            let cost = self.cost_calculator.calculate(&solution);
                            new_solutions.push((solution, cost));
                        }
                    }
                }
            }

            solutions.extend(new_solutions.drain(..));
        }

        solutions
            .into_iter()
            .min_by_key(|(_, one)| OrderedFloat(*one))
            .expect("at least one solution")
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cost::{DeliveryCostCalculator, DistanceCostCalculator},
        route::CrowRouter,
        Location, SimpleProblem, Stop, Vehicle,
    };

    const DISTANCE_COST: f64 = 1.0;
    const MISSED_DELIVERY_COST: f64 = 1e9;

    static ROUTER: CrowRouter = CrowRouter::new();

    fn solve(problem: &SimpleProblem) -> Solution {
        BranchAndBoundSolver::new(DeliveryCostCalculator::new(
            DistanceCostCalculator::new(&ROUTER, problem),
            problem.stops().len(),
            MISSED_DELIVERY_COST,
            DISTANCE_COST,
        ))
        .solve(problem)
    }

    #[test]
    fn do_nothing() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new(0, 1)],
            vec![],
            vec![Location::new(0.0, 0.0), Location::new(1.0, 0.0)],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![].into()]));
    }

    #[test]
    fn keep_one_stop() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new(0, 2)],
            vec![Stop::new(1)],
            vec![
                Location::new(0.0, 0.0),
                Location::new(1.0, 0.0),
                Location::new(2.0, 0.0),
            ],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![0].into()]));
    }

    #[test]
    fn keep_two_stops() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new(0, 3)],
            vec![Stop::new(1), Stop::new(2)],
            vec![
                Location::new(0.0, 0.0),
                Location::new(1.0, 0.0),
                Location::new(2.0, 0.0),
                Location::new(3.0, 0.0),
            ],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![0, 1].into()]));
    }

    #[test]
    fn keep_three_stops() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new(0, 4)],
            vec![Stop::new(1), Stop::new(2), Stop::new(3)],
            vec![
                Location::new(0.0, 0.0),
                Location::new(1.0, 0.0),
                Location::new(2.0, 0.0),
                Location::new(3.0, 0.0),
                Location::new(4.0, 0.0),
            ],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![0, 1, 2].into()]));
    }

    #[test]
    fn even_workload() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new(0, 2), Vehicle::new(3, 5)],
            vec![Stop::new(1), Stop::new(4)],
            vec![
                Location::new(0.0, 0.0),
                Location::new(1.0, 0.0),
                Location::new(2.0, 0.0),
                Location::new(0.0, 1.0),
                Location::new(1.0, 1.0),
                Location::new(2.0, 1.0),
            ],
        );

        assert!(solve(&problem)
            .routes()
            .iter()
            .all(|stops| stops.len() == 1));
    }
}
