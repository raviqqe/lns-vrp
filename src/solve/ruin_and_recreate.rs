use super::solver::Solver;
use crate::{cost::CostCalculator, hash_map::HashMap, problem::BaseProblem, Solution};
use ordered_float::OrderedFloat;
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};
use std::ops::Range;

const SEED: [u8; 32] = [0u8; 32];
const MAX_STOP_RANGE_SIZE: usize = 3;

#[derive(Debug)]
struct RouteRegion {
    vehicle_index: usize,
    range: Range<usize>,
}

#[derive(Debug)]
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
                range: self.choose_range(solution, vehicle_index),
            })
        } else {
            let mut vehicle_indexes = [0usize; 2];

            (0..solution.routes().len()).choose_multiple_fill(&mut self.rng, &mut vehicle_indexes);

            Region::Two(
                RouteRegion {
                    vehicle_index: vehicle_indexes[0],
                    range: self.choose_range(solution, vehicle_indexes[0]),
                },
                RouteRegion {
                    vehicle_index: vehicle_indexes[1],
                    range: self.choose_range(solution, vehicle_indexes[1]),
                },
            )
        }
    }

    fn choose_range(&mut self, solution: &Solution, vehicle_index: usize) -> Range<usize> {
        let len = solution.routes()[vehicle_index].len();
        let index = (0..len.saturating_sub(MAX_STOP_RANGE_SIZE))
            .choose(&mut self.rng)
            .unwrap_or(0);

        index..(index + MAX_STOP_RANGE_SIZE).min(len)
    }

    fn solve_one_region(&mut self, initial_solution: &Solution, region: &RouteRegion) -> Solution {
        let solution = initial_solution.ruin_route(region.vehicle_index, region.range.clone());
        let cost = self.cost_calculator.calculate(&solution);

        let mut solutions = HashMap::default();
        solutions.insert(solution, cost);
        let mut new_solutions = vec![];

        for _ in region.range.clone() {
            new_solutions.clear();

            for solution in solutions.keys() {
                for stop_index in Self::region_stop_indexes(region, initial_solution) {
                    if solution.has_stop(stop_index) {
                        continue;
                    }

                    let solution =
                        solution.insert_stop(region.vehicle_index, region.range.start, stop_index);
                    let cost = self.cost_calculator.calculate(&solution);

                    if cost.is_finite() {
                        new_solutions.push((solution, cost));
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

    fn solve_two_regions(
        &mut self,
        initial_solution: &Solution,
        one: &RouteRegion,
        other: &RouteRegion,
    ) -> Solution {
        let solution = initial_solution
            .ruin_route(one.vehicle_index, one.range.clone())
            .ruin_route(other.vehicle_index, other.range.clone());
        let cost = self.cost_calculator.calculate(&solution);

        let mut solutions = HashMap::default();
        solutions.insert(solution.clone(), cost);
        let mut new_solutions = vec![];

        for _ in [one, other].iter().flat_map(|region| region.range.clone()) {
            new_solutions.clear();

            for solution in solutions.keys() {
                for stop_index in [one, other]
                    .iter()
                    .flat_map(|region| Self::region_stop_indexes(region, initial_solution))
                {
                    if solution.has_stop(stop_index) {
                        continue;
                    }

                    for region in [one, other] {
                        let solution = solution.insert_stop(
                            region.vehicle_index,
                            region.range.start,
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
            .range
            .clone()
            .map(|index| solution.routes()[region.vehicle_index][index])
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
            let new_solution = match &self.choose_region(&solution) {
                Region::One(region) => self.solve_one_region(&solution, region),
                Region::Two(one, other) => self.solve_two_regions(&solution, one, other),
            };
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
        Location, SimpleProblem, Stop, Vehicle,
    };

    const DISTANCE_COST: f64 = 1.0;
    const MISSED_DELIVERY_COST: f64 = 1e9;
    const ITERATION_COUNT: usize = 100;

    fn solve(problem: &SimpleProblem) -> Solution {
        RuinAndRecreateSolver::new(
            DeliveryCostCalculator::new(
                DistanceCostCalculator::new(problem),
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
