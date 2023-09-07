use super::solver::Solver;
use crate::{
    cost::CostCalculator, hash_map::HashMap, problem::BaseProblem, route::Router, trace, Solution,
};
use bumpalo::Bump;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};
use std::{alloc::Global, ops::Range};

const SEED: [u8; 32] = [0u8; 32];
const MAX_STOP_REGION_SIZE: usize = 6;
const CLOSEST_STOP_COUNT: usize = 4;

#[derive(Debug)]
struct RouteRegion {
    vehicle_index: usize,
    stop_range: Range<usize>,
}

pub struct RuinAndRecreateSolver<C: CostCalculator, R: Router, S: Solver> {
    initial_solver: S,
    cost_calculator: C,
    router: R,
    iteration_count: usize,
    rng: SmallRng,
}

impl<C: CostCalculator, R: Router, S: Solver> RuinAndRecreateSolver<C, R, S> {
    pub fn new(cost_calculator: C, router: R, initial_solver: S, iteration_count: usize) -> Self {
        Self {
            initial_solver,
            cost_calculator,
            router,
            iteration_count,
            rng: SmallRng::from_seed(SEED),
        }
    }

    fn choose_regions(
        &mut self,
        solution: &Solution,
        closest_stops: &[Vec<usize>],
    ) -> Vec<RouteRegion> {
        let (stop_index, closest_stop_indexes) = closest_stops
            .iter()
            .enumerate()
            .choose(&mut self.rng)
            .expect("stop pair");

        let pairs = [stop_index]
            .iter()
            .chain(closest_stop_indexes)
            .copied()
            .flat_map(|stop_index| {
                Self::find_vehicle(solution, stop_index)
                    .map(|vehicle_index| (vehicle_index, stop_index))
            })
            .unique_by(|(vehicle_index, _)| *vehicle_index)
            .collect::<Vec<_>>();

        pairs
            .iter()
            .map(|(vehicle_index, stop_index)| {
                self.choose_region(
                    solution,
                    *vehicle_index,
                    *stop_index,
                    MAX_STOP_REGION_SIZE / pairs.len(),
                )
            })
            .collect()
    }
    fn find_vehicle(solution: &Solution, stop_index: usize) -> Option<usize> {
        solution
            .routes()
            .iter()
            .enumerate()
            .find_map(|(vehicle_index, route)| route.contains(&stop_index).then_some(vehicle_index))
    }

    fn choose_region(
        &mut self,
        solution: &Solution,
        vehicle_index: usize,
        stop_index: usize,
        stop_region_size: usize,
    ) -> RouteRegion {
        let route = &solution.routes()[vehicle_index];
        let middle = route
            .iter()
            .position(|&other| other == stop_index)
            .expect("stop index");
        let start = middle.saturating_sub(stop_region_size / 2);

        RouteRegion {
            vehicle_index,
            stop_range: start..(start + stop_region_size).min(route.len()),
        }
    }

    fn optimize_regions(
        &mut self,
        initial_solution: &Solution,
        regions: &[RouteRegion],
    ) -> Solution {
        let bump = Bump::new();
        let mut solution = initial_solution.clone_in(&bump);

        for region in regions {
            solution = solution.drain_route(region.vehicle_index, region.stop_range.clone())
        }

        let cost = self.cost_calculator.calculate(&solution);

        let mut solutions = HashMap::default();
        solutions.insert(solution.clone(), cost);
        let mut new_solutions = vec![];

        for _ in regions.iter().flat_map(|region| region.stop_range.clone()) {
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

        let solution = solutions
            .into_iter()
            .min_by(|(_, one), (_, other)| OrderedFloat(*one).cmp(&OrderedFloat(*other)))
            .expect("at least one solution")
            .0;

        solution.clone_in(Global)
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

    fn swap_heads_and_tails(
        &mut self,
        initial_solution: &Solution,
        closest_stops: &[Vec<usize>],
    ) -> Option<(Solution, f64)> {
        let (stop_index, stops) = closest_stops
            .iter()
            .enumerate()
            .choose(&mut self.rng)
            .expect("at least one route");
        let stop_indexes = [
            stop_index,
            *stops
                .iter()
                .choose(&mut self.rng)
                .expect("at least one closest stop"),
        ];
        let vehicle_indexes = stop_indexes
            .iter()
            .flat_map(|&stop_index| Self::find_vehicle(initial_solution, stop_index))
            .collect::<Vec<_>>();

        if vehicle_indexes.len() < 2 {
            return None;
        }

        let mut base_solution = initial_solution.clone();

        for vehicle_index in vehicle_indexes {
            base_solution = base_solution.drain_route(
                vehicle_index,
                0..base_solution.routes()[vehicle_index].len(),
            );
        }

        let mut solution = initial_solution.clone();
        let mut cost = self.cost_calculator.calculate(&solution);

        for head_source in 0..2 {
            for head_target in 0..2 {
                for tail_source in 0..2 {
                    for tail_target in 0..2 {
                        let mut new_solution = base_solution.clone();

                        for (source, target) in [
                            (head_source, head_target),
                            (1 - head_source, 1 - head_target),
                        ] {
                            new_solution = new_solution.extend_route(
                                target,
                                initial_solution.routes()[source][..stop_indexes[head_source]]
                                    .iter()
                                    .copied(),
                            );
                        }

                        for (source, target) in [
                            (tail_source, tail_target),
                            (1 - tail_source, 1 - tail_target),
                        ] {
                            new_solution = new_solution.extend_route(
                                target,
                                initial_solution.routes()[source][stop_indexes[head_source]..]
                                    .iter()
                                    .copied(),
                            );
                        }

                        let new_cost = self.cost_calculator.calculate(&new_solution);

                        if new_cost < cost {
                            solution = new_solution;
                            cost = new_cost;
                        }
                    }
                }
            }
        }

        Some((solution, cost))
    }
}

impl<C: CostCalculator, R: Router, S: Solver> Solver for RuinAndRecreateSolver<C, R, S> {
    fn solve(&mut self, problem: impl BaseProblem) -> Solution {
        if problem.vehicle_count() == 0 {
            return Solution::new(vec![]);
        } else if problem.stop_count() == 0 {
            return Solution::new(
                (0..problem.vehicle_count())
                    .map(|_| vec![].into())
                    .collect(),
            );
        } else if problem.stop_count() == 1 {
            return self.initial_solver.solve(problem);
        }

        let closest_stops = ((0..problem.stop_count()).map(|one| {
            let mut stops = (0..problem.stop_count())
                .filter(|other| one != *other)
                .collect::<Vec<_>>();

            stops.sort_by_key(|&other| {
                OrderedFloat(self.router.route(
                    problem.location(problem.stop_location(one)),
                    problem.location(problem.stop_location(other)),
                ))
            });
            stops.truncate(CLOSEST_STOP_COUNT);

            stops
        }))
        .collect::<Vec<_>>();

        let mut solution = self.initial_solver.solve(problem);
        let mut cost = self.cost_calculator.calculate(&solution);

        for _ in 0..self.iteration_count {
            if let Some((new_solution, new_cost)) =
                self.swap_heads_and_tails(&solution, &closest_stops)
            {
                solution = new_solution;
                cost = new_cost;
            }

            let regions = self.choose_regions(&solution, &closest_stops);
            trace!("regions: {:?}", &regions);
            let new_solution = self.optimize_regions(&solution, &regions);
            let new_cost = self.cost_calculator.calculate(&new_solution);

            // TODO Consider a non-greedy strategy like simulated annealing.
            // TODO Save multiple solutions.
            // TODO Decide if a solution is good enough already.
            if new_cost < cost {
                trace!("new solution found!");
                trace!("solution: {:?}", solution);
                trace!("cost: {:?}", cost);

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
        solve::NearestNeighborSolver,
        Location, SimpleProblem, Stop, Vehicle,
    };

    const DISTANCE_COST: f64 = 1.0;
    const MISSED_DELIVERY_COST: f64 = 1e9;
    const ITERATION_COUNT: usize = 100;

    static ROUTER: CrowRouter = CrowRouter::new();

    fn solve(problem: &SimpleProblem) -> Solution {
        RuinAndRecreateSolver::new(
            DeliveryCostCalculator::new(
                DistanceCostCalculator::new(&ROUTER, problem),
                problem.stops().len(),
                MISSED_DELIVERY_COST,
                DISTANCE_COST,
            ),
            &ROUTER,
            NearestNeighborSolver::new(&ROUTER),
            ITERATION_COUNT,
        )
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
