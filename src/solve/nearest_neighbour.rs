use super::solver::Solver;
use crate::{problem::BaseProblem, route::Router, Solution};
use ordered_float::OrderedFloat;
use std::collections::HashSet;

pub struct NearestNeighbourSolver<R: Router> {
    router: R,
}

impl<R: Router> NearestNeighbourSolver<R> {
    pub fn new(router: R) -> Self {
        Self { router }
    }
}

impl<R: Router> Solver for NearestNeighbourSolver<R> {
    fn solve(&mut self, problem: impl BaseProblem) -> Solution {
        if problem.vehicle_count() == 0 {
            return Solution::new(vec![]);
        }

        let mut solution = Solution::new(
            (0..problem.vehicle_count())
                .map(|_| vec![].into())
                .collect(),
        );
        let mut stops = HashSet::<usize>::from_iter(0..problem.stop_count());

        for index in 0..problem.vehicle_count().min(problem.stop_count()) {
            solution = solution.add_stop(index, index);
            stops.remove(&index);
        }

        loop {
            for vehicle_index in 0..problem.vehicle_count() {
                if stops.is_empty() {
                    return solution;
                }

                let last_location = problem.stop_location(
                    *solution.routes()[vehicle_index]
                        .last()
                        .expect("last location"),
                );

                let stop_index = stops
                    .iter()
                    .map(|&index| {
                        (
                            index,
                            self.router
                                .route(last_location, problem.stop_location(index)),
                        )
                    })
                    .min_by_key(|(_, distance)| OrderedFloat(*distance))
                    .expect("stop index")
                    .0;

                solution = solution.add_stop(vehicle_index, stop_index);
                stops.remove(&stop_index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{route::CrowRouter, Location, SimpleProblem, Stop, Vehicle};

    fn solve(problem: &SimpleProblem) -> Solution {
        NearestNeighbourSolver::new(CrowRouter::new()).solve(problem)
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
    fn distribute_to_two_vehicles() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new(), Vehicle::new()],
            vec![
                Stop::new(Location::new(0.0, 0.0)),
                Stop::new(Location::new(1.0, 0.0)),
                Stop::new(Location::new(0.1, 0.0)),
                Stop::new(Location::new(1.1, 0.0)),
                Stop::new(Location::new(0.2, 0.0)),
                Stop::new(Location::new(1.3, 0.0)),
            ],
        );

        assert_eq!(
            solve(&problem),
            Solution::new(vec![vec![0, 2, 4].into(), vec![1, 3, 5].into()])
        );
    }

    #[test]
    fn distribute_to_two_vehicles_with_uneven_stops() {
        let problem = SimpleProblem::new(
            vec![Vehicle::new(), Vehicle::new()],
            vec![
                Stop::new(Location::new(0.0, 0.0)),
                Stop::new(Location::new(1.0, 0.0)),
                Stop::new(Location::new(0.1, 0.0)),
                Stop::new(Location::new(1.1, 0.0)),
                Stop::new(Location::new(0.2, 0.0)),
            ],
        );

        assert_eq!(
            solve(&problem),
            Solution::new(vec![vec![0, 2, 4].into(), vec![1, 3].into()])
        );
    }
}
