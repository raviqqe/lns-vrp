use super::solver::Solver;
use crate::{cost::CostCalculator, problem::BaseProblem, Solution};
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};
use std::ops::Range;

const SEED: [u8; 32] = [0u8; 32];

struct RouteRegion {
    vehicle_index: usize,
    range: Range<usize>,
}

enum Region {
    One(RouteRegion),
    Two(RouteRegion, RouteRegion),
}

pub struct RuinAndRecreateSolver<C: CostCalculator> {
    cost_calculator: C,
    iteration_count: usize,
    rng: SmallRng,
}

impl<C: CostCalculator> RuinAndRecreateSolver<C> {
    pub fn new(cost_calculator: C, iteration_count: usize) -> Self {
        Self {
            cost_calculator,
            iteration_count,
            rng: SmallRng::from_seed(SEED),
        }
    }

    fn choose_region(&mut self, solution: &Solution) -> Region {
        if rand::random::<bool>() || solution.routes().len() == 1 {
            let vehicle_index = (0..solution.routes().len())
                .choose(&mut self.rng)
                .expect("at least one vehicle");

            Region::One(RouteRegion {
                vehicle_index,
                range: todo!(),
            })
        } else {
            let mut vehicle_indexes = [0usize; 2];

            (0..solution.routes().len()).choose_multiple_fill(&mut self.rng, &mut vehicle_indexes);

            Region::Two(
                RouteRegion {
                    vehicle_index: vehicle_indexes[0],
                    range: todo!(),
                },
                RouteRegion {
                    vehicle_index: vehicle_indexes[1],
                    range: todo!(),
                },
            )
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

            let new_solution = todo!();

            if self.cost_calculator.calculate(&new_solution) < cost {
                solution = new_solution;
            }
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
