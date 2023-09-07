use super::solver::Solver;
use crate::{
    cost::CostCalculator, hash_map::HashMap, problem::BaseProblem, route::Router, trace,
    trace_solution, Solution,
};
use bumpalo::Bump;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};
use std::{alloc::Global, ops::Range};

const SEED: [u8; 32] = [0u8; 32];

const MAX_FACTORIAL_SUB_PROBLEM_SIZE: usize = 8;
const MAX_VEHICLE_REGION_SIZE: usize = 2;

const TWO_OPT_MAX_STOP_COUNT: usize = 10;

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

    fn run_dynamic_programming(
        &mut self,
        solution: &Solution,
        closest_stops: &[Vec<usize>],
    ) -> Solution {
        let regions = self.choose_regions(solution, closest_stops);
        trace!("regions: {:?}", &regions);

        self.optimize_regions(solution, &regions)
    }

    fn choose_regions(
        &mut self,
        solution: &Solution,
        closest_stops: &[Vec<usize>],
    ) -> Vec<RouteRegion> {
        let vehicle_count = (1..MAX_VEHICLE_REGION_SIZE + 1)
            .choose(&mut self.rng)
            .expect("vehicle count");

        let (stop_index, closest_stop_indexes) = closest_stops
            .iter()
            .enumerate()
            .choose(&mut self.rng)
            .expect("stop pair");
        let pairs = [stop_index]
            .iter()
            .chain(closest_stop_indexes)
            .copied()
            .choose_multiple(&mut self.rng, vehicle_count)
            .into_iter()
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
                    (MAX_FACTORIAL_SUB_PROBLEM_SIZE - vehicle_count) / pairs.len(),
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

            // Keep existing solutions with stops not assigned.
            solutions.extend(new_solutions.drain(..));
        }

        let (solution, _) = solutions
            .into_iter()
            .min_by_key(|(_, cost)| OrderedFloat(*cost))
            .expect("at least one solution");

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

    fn run_two_opt(
        &mut self,
        initial_solution: &Solution,
        closest_stops: &[Vec<usize>],
    ) -> Solution {
        let (stop_index, stops) = closest_stops
            .iter()
            .enumerate()
            .choose(&mut self.rng)
            .expect("at least one route");

        let mut solution = initial_solution.clone();
        let mut cost = self.cost_calculator.calculate(initial_solution);

        for stop_indexes in [stop_index]
            .iter()
            .chain(stops)
            .take(TWO_OPT_MAX_STOP_COUNT)
            .copied()
            .combinations(2)
            .filter(|indexes| indexes[0] != indexes[1])
        {
            let vehicle_indexes = stop_indexes
                .iter()
                .flat_map(|&stop_index| Self::find_vehicle(initial_solution, stop_index))
                .unique()
                .collect::<Vec<_>>();

            if vehicle_indexes.len() == 2 {
                let (new_solution, new_cost) =
                    self.run_inter_route_two_opt(initial_solution, &vehicle_indexes, &stop_indexes);

                if new_cost < cost {
                    trace_solution!("2-opt", &solution, cost);

                    solution = new_solution;
                    cost = new_cost;
                }
            }

            // TODO Apply the intra-route 2-opt heuristics.
        }

        solution
    }

    fn run_inter_route_two_opt(
        &mut self,
        initial_solution: &Solution,
        vehicle_indexes: &[usize],
        stop_indexes: &[usize],
    ) -> (Solution, f64) {
        let mut solution = initial_solution.clone();
        let mut cost = self.cost_calculator.calculate(initial_solution);

        for initial_solution in [false, true].into_iter().permutations(2).map(|flags| {
            let mut solution = initial_solution.clone();

            for (index, flag) in flags.into_iter().enumerate() {
                if flag {
                    solution = solution.reverse_route(vehicle_indexes[index]);
                }
            }

            solution
        }) {
            let vehicles = vehicle_indexes
                .iter()
                .zip(stop_indexes)
                .map(|(&vehicle_index, &stop_index)| {
                    (
                        vehicle_index,
                        initial_solution.routes()[vehicle_index]
                            .iter()
                            .position(|&other| other == stop_index)
                            .expect("existent stop index"),
                    )
                })
                .collect::<Vec<_>>();

            for vehicles in [
                // Inclusive stop index
                vehicles
                    .iter()
                    .map(|(vehicle_index, stop_index)| (*vehicle_index, stop_index + 1))
                    .collect::<Vec<_>>(),
                // Exclusive stop index
                vehicles,
            ] {
                let new_solution = {
                    let mut solution = initial_solution.clone();

                    for &(vehicle_index, _) in &vehicles {
                        solution = solution
                            .drain_route(vehicle_index, 0..solution.routes()[vehicle_index].len());
                    }

                    solution
                };

                for head_source in 0..2 {
                    for head_target in 0..2 {
                        let new_solution = Self::extend_routes(
                            &initial_solution,
                            &new_solution,
                            &vehicles,
                            head_source,
                            head_target,
                            false,
                        );

                        for tail_source in 0..2 {
                            for tail_target in 0..2 {
                                let new_solution = Self::extend_routes(
                                    &initial_solution,
                                    &new_solution,
                                    &vehicles,
                                    tail_source,
                                    tail_target,
                                    true,
                                );
                                let new_cost = self.cost_calculator.calculate(&new_solution);

                                if new_cost < cost {
                                    solution = new_solution;
                                    cost = new_cost;
                                }
                            }
                        }
                    }
                }
            }
        }

        (solution, cost)
    }

    fn extend_routes(
        initial_solution: &Solution,
        solution: &Solution,
        vehicles: &[(usize, usize)],
        source: usize,
        target: usize,
        tail: bool,
    ) -> Solution {
        let mut solution = solution.clone();

        for (source, target) in [(source, target), (1 - source, 1 - target)] {
            let &(source_vehicle_index, source_stop_index) = &vehicles[source];
            let &(target_vehicle_index, _) = &vehicles[target];
            let source_route = &initial_solution.routes()[source_vehicle_index];

            solution = solution.extend_route(
                target_vehicle_index,
                if tail {
                    &source_route[source_stop_index..]
                } else {
                    &source_route[..source_stop_index]
                }
                .iter()
                .copied(),
            );
        }

        solution
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

            stops
        }))
        .collect::<Vec<_>>();

        let mut update_delta = 0.0;
        let mut delta = 0.0;
        let mut solution = self.initial_solver.solve(problem);
        let mut cost = self.cost_calculator.calculate(&solution);

        while delta < update_delta / 100 {
            solution = self.run_two_opt(&solution, &closest_stops);
            solution = self.run_dynamic_programming(&solution, &closest_stops);

            let new_cost = self.cost_calculator.calculate(&solution);

            if cost < new_cost {
                foo
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
    const QUADRATIC_DISTANCE_COST: f64 = 1e-3;
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
                QUADRATIC_DISTANCE_COST,
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
