use super::solver::Solver;
use crate::{cost::CostCalculator, hash_map::HashMap, problem::BaseProblem, Solution};
use bumpalo::Bump;
use ordered_float::OrderedFloat;

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
    fn solve(&mut self, problem: impl BaseProblem) -> Solution {
        let allocator = Bump::new();
        let mut solutions = HashMap::default();
        let solution = Solution::new({
            let mut routes = Vec::with_capacity_in(problem.vehicle_count(), &allocator);
            routes.extend((0..problem.vehicle_count()).map(|_| Vec::new_in(&allocator).into()));
            routes
        });
        let cost = self.cost_calculator.calculate(&solution);
        solutions.insert(solution, cost);
        let mut new_solutions = vec![];

        for _ in 0..problem.stop_count() {
            new_solutions.clear();

            for solution in solutions.keys() {
                for stop_index in 0..problem.stop_count() {
                    if solution.has_stop(stop_index) {
                        continue;
                    }

                    for vehicle_index in 0..solution.routes().len() {
                        let solution = solution.add_stop(vehicle_index, stop_index);
                        let cost = self.cost_calculator.calculate(&solution);

                        if cost.is_finite() {
                            new_solutions.push((solution, cost));
                        }
                    }
                }
            }

            solutions.extend(new_solutions.drain(..));
        }

        let solution = solutions
            .into_iter()
            .min_by(|(_, one), (_, other)| OrderedFloat(*one).cmp(&OrderedFloat(*other)))
            .expect("at least one solution")
            .0;

        solution.to_global()
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

    fn solve(problem: &SimpleProblem) -> Solution {
        DynamicProgrammingSolver::new(DeliveryCostCalculator::new(
            DistanceCostCalculator::new(CrowRouter::new(), problem),
            problem.stops().len(),
            MISSED_DELIVERY_COST,
            DISTANCE_COST,
        ))
        .solve(problem)
    }

    #[test]
    fn do_nothing() {
        let problem = SimpleProblem::new(vec![Vehicle::new()], vec![]);

        assert_eq!(solve(&problem), Solution::new(vec![vec![].into()]));
    }

    #[test]
    fn keep_one_stop() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new()],
            vec![Stop::new(Location::new(0.0, 0.0))],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![0].into()]));
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
