use super::solver::Solver;
use crate::{
    cost::CostCalculator,
    hash_map::HashMap,
    intermediate_solution::{IntermediateRoute, IntermediateSolution},
    problem::BaseProblem,
    Solution,
};
use bumpalo::Bump;

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
        let solution = IntermediateSolution::new({
            let mut routes = Vec::with_capacity_in(problem.vehicle_count(), &allocator);
            routes.extend((0..problem.vehicle_count()).map(|_| IntermediateRoute::new(0, 0)));
            routes
        });
        solutions.insert(solution, 0.0);
        let mut new_solutions = vec![];

        // for _ in 0..problem.stop_count() {
        //     for solution in solutions.keys() {
        //         for stop_index in 0..problem.stop_count() {
        //             if solution.has_stop(stop_index) {
        //                 continue;
        //             }

        //             for vehicle_index in 0..solution.routes().len() {
        //                 let solution = solution.add_stop(vehicle_index,
        // stop_index);                 let cost =
        // self.cost_calculator.calculate(&solution);

        //                 if cost.is_finite() {
        //                     new_solutions.push((solution, cost));
        //                 }
        //             }
        //         }
        //     }

        //     solutions.extend(new_solutions.drain(..));
        // }

        // let solution = solutions
        //     .into_iter()
        //     .min_by_key(|(_, cost)| OrderedFloat(*cost))
        //     .expect("at least one solution")
        //     .0;

        // solution.clone_in(Global)

        todo!()
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
    const QUADRATIC_DISTANCE_COST: f64 = 1e-3;
    const MISSED_DELIVERY_COST: f64 = 1e9;

    static ROUTER: CrowRouter = CrowRouter::new();

    fn solve(problem: &SimpleProblem) -> Solution {
        DynamicProgrammingSolver::new(DeliveryCostCalculator::new(
            DistanceCostCalculator::new(&ROUTER, problem),
            problem.stops().len(),
            MISSED_DELIVERY_COST,
            DISTANCE_COST,
            QUADRATIC_DISTANCE_COST,
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
