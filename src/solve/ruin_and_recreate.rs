use super::solver::Solver;
use crate::{cost::CostCalculator, hash_map::HashMap, problem::BaseProblem, Solution};
use ordered_float::OrderedFloat;
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};
use std::ops::Range;

const SEED: [u8; 32] = [0u8; 32];
const MAX_VEHICLE_REGION_SIZE: usize = 2;
const MAX_STOP_REGION_SIZE: usize = 3;

#[derive(Debug)]
struct RouteRegion {
    vehicle_index: usize,
    stop_range: Range<usize>,
}

pub struct RuinAndRecreateSolver<C: CostCalculator, S: Solver> {
    cost_calculator: C,
    initial_solver: S,
    iteration_count: usize,
    rng: SmallRng,
}

impl<C: CostCalculator, S: Solver> RuinAndRecreateSolver<C, S> {
    pub fn new(cost_calculator: C, initial_solver: S, iteration_count: usize) -> Self {
        Self {
            cost_calculator,
            initial_solver,
            iteration_count,
            rng: SmallRng::from_seed(SEED),
        }
    }

    fn choose_regions(&mut self, solution: &Solution) -> Vec<RouteRegion> {
        let vehicle_count = solution.routes().len();
        let route_count = (1.min(vehicle_count)..(MAX_VEHICLE_REGION_SIZE.min(vehicle_count) + 1))
            .choose(&mut self.rng)
            .expect("ruined route count");

        (0..vehicle_count)
            .choose_multiple(&mut self.rng, route_count)
            .into_iter()
            .map(|vehicle_index| RouteRegion {
                vehicle_index,
                stop_range: self.choose_range(solution, vehicle_index),
            })
            .collect()
    }

    fn choose_range(&mut self, solution: &Solution, vehicle_index: usize) -> Range<usize> {
        let len = solution.routes()[vehicle_index].len();
        let index = (0..len.saturating_sub(MAX_STOP_REGION_SIZE))
            .choose(&mut self.rng)
            .unwrap_or(0);

        index..(index + MAX_STOP_REGION_SIZE).min(len)
    }

    fn optimize_regions(
        &mut self,
        initial_solution: &Solution,
        regions: &[RouteRegion],
    ) -> Solution {
        let mut solution = initial_solution.clone();

        for region in regions {
            solution = solution.ruin_route(region.vehicle_index, region.stop_range.clone())
        }

        let cost = self.cost_calculator.calculate(&solution);

        let mut solutions = HashMap::default();
        solutions.insert(solution.clone(), cost);
        let mut new_solutions = vec![];

        for _ in regions.iter().flat_map(|region| region.stop_range.clone()) {
            new_solutions.clear();

            for solution in solutions.keys() {
                for stop_index in regions
                    .iter()
                    .flat_map(|region| Self::region_stop_indexes(region, initial_solution))
                {
                    if solution.has_stop(stop_index) {
                        continue;
                    }

                    for region in regions {
                        let solution = solution.insert_stop(
                            region.vehicle_index,
                            region.stop_range.start,
                            stop_index,
                        );
                        let cost = self.cost_calculator.calculate(&solution);

                        if cost.is_finite() {
                            new_solutions.push((solution, cost));
                        }
                    }
                }
            }

            solutions.extend(new_solutions.drain(..));
        }

        solutions
            .into_iter()
            .min_by(|(_, one), (_, other)| OrderedFloat(*one).cmp(&OrderedFloat(*other)))
            .expect("at least one solution")
            .0
    }

    fn region_stop_indexes<'a>(
        region: &'a RouteRegion,
        solution: &'a Solution,
    ) -> impl Iterator<Item = usize> + 'a {
        region
            .stop_range
            .clone()
            .map(|index| solution.routes()[region.vehicle_index][index])
    }
}

impl<C: CostCalculator> Solver for RuinAndRecreateSolver<C> {
    fn solve(&mut self, problem: impl BaseProblem) -> Solution {
        if problem.vehicle_count() == 0 {
            return Solution::new(vec![]);
        }

        // TODO Build an initial solution with heuristics.
        let mut solution = self.initial_solver.solve(problem);
        let mut cost = self.cost_calculator.calculate(&solution);

        for _ in 0..self.iteration_count {
            let regions = self.choose_regions(&solution);
            let new_solution = self.optimize_regions(&solution, &regions);
            let new_cost = self.cost_calculator.calculate(&new_solution);

            // TODO Consider a non-greedy strategy like simulated annealing.
            // TODO Save multiple solutions.
            // TODO Decide if a solution is good enough already.
            if new_cost < cost {
                solution = new_solution;
                cost = new_cost;
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
        route::CrowRouter,
        Location, SimpleProblem, Stop, Vehicle,
    };

    const DISTANCE_COST: f64 = 1.0;
    const MISSED_DELIVERY_COST: f64 = 1e9;
    const ITERATION_COUNT: usize = 100;

    fn solve(problem: &SimpleProblem) -> Solution {
        RuinAndRecreateSolver::new(
            DeliveryCostCalculator::new(
                DistanceCostCalculator::new(CrowRouter::new(), problem),
                problem.stops().len(),
                MISSED_DELIVERY_COST,
                DISTANCE_COST,
            ),
            ITERATION_COUNT,
        )
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

        assert_eq!(solve(&problem), Solution::new(vec![vec![0, 1].into()]));
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

        assert_eq!(solve(&problem), Solution::new(vec![vec![0, 1, 2].into()]));
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
