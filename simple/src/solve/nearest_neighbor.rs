use crate::{route::Router, Problem, Solution, Stop, Vehicle};
use core::{BasicProblem, BasicSolver, BasicStop, BasicVehicle};
use ordered_float::OrderedFloat;
use std::collections::HashSet;

pub struct NearestNeighborSolver<R: Router> {
    router: R,
}

impl<R: Router> NearestNeighborSolver<R> {
    pub fn new(router: R) -> Self {
        Self { router }
    }
}

impl<R: Router> BasicSolver<Vehicle, Stop, Problem, Solution> for NearestNeighborSolver<R> {
    fn solve(&mut self, problem: &Problem) -> Solution {
        if problem.vehicle_count() == 0 {
            return Solution::new(vec![]);
        }

        let mut solution = Solution::new(
            (0..problem.vehicle_count())
                .map(|_| vec![].into())
                .collect(),
        );
        let mut stops = HashSet::<usize>::from_iter(0..problem.stop_count());

        loop {
            for vehicle_index in 0..problem.vehicle_count() {
                if stops.is_empty() {
                    return solution;
                }

                let last_location = problem.location(
                    if let Some(&stop_index) = solution.routes()[vehicle_index].last() {
                        problem.stop(stop_index).location()
                    } else {
                        problem.vehicle(vehicle_index).start_location()
                    },
                );

                let stop_index = stops
                    .iter()
                    .copied()
                    .min_by_key(|index| {
                        OrderedFloat(self.router.route(
                            last_location,
                            problem.location(problem.stop(*index).location()),
                        ))
                    })
                    .expect("stop index");

                solution = solution.add_stop(vehicle_index, stop_index);
                stops.remove(&stop_index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{route::CrowRouter, Problem, Stop, Vehicle};
    use core::Location;

    static ROUTER: CrowRouter = CrowRouter::new();

    fn solve(problem: &Problem) -> Solution {
        NearestNeighborSolver::new(&ROUTER).solve(problem)
    }

    #[test]
    fn do_nothing() {
        let problem = Problem::new(
            vec![Vehicle::new(0, 0)],
            vec![],
            vec![Location::new(0.0, 0.0)],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![].into()]));
    }

    #[test]
    fn keep_one_stop() {
        let problem = Problem::new(
            vec![Vehicle::new(0, 0)],
            vec![Stop::new(0)],
            vec![Location::new(0.0, 0.0)],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![0].into()]));
    }

    #[test]
    fn keep_two_stops() {
        let problem = Problem::new(
            vec![Vehicle::new(0, 0)],
            vec![Stop::new(0), Stop::new(1)],
            vec![Location::new(0.0, 0.0), Location::new(1.0, 0.0)],
        );

        assert_eq!(solve(&problem).routes()[0].len(), 2);
    }

    #[test]
    fn keep_three_stops() {
        let problem = Problem::new(
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

        assert_eq!(solve(&problem).routes()[0][1], 1);
    }

    #[test]
    fn optimize_stop_order() {
        let problem = Problem::new(
            vec![Vehicle::new(0, 4)],
            vec![Stop::new(1), Stop::new(3), Stop::new(2)],
            vec![
                Location::new(0.0, 0.0),
                Location::new(1.0, 0.0),
                Location::new(2.0, 0.0),
                Location::new(3.0, 0.0),
                Location::new(4.0, 0.0),
            ],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![0, 2, 1].into()]));
    }

    #[test]
    fn distribute_to_two_vehicles() {
        let problem = Problem::new(
            vec![Vehicle::new(0, 0), Vehicle::new(4, 4)],
            vec![
                Stop::new(1),
                Stop::new(2),
                Stop::new(3),
                Stop::new(5),
                Stop::new(6),
                Stop::new(7),
            ],
            vec![
                Location::new(0.0, 0.0),
                Location::new(0.1, 0.0),
                Location::new(0.2, 0.0),
                Location::new(0.3, 0.0),
                Location::new(0.0, 1.0),
                Location::new(0.1, 1.0),
                Location::new(0.2, 1.0),
                Location::new(0.3, 1.0),
            ],
        );

        assert_eq!(
            solve(&problem),
            Solution::new(vec![vec![0, 1, 2].into(), vec![3, 4, 5].into()])
        );
    }

    #[test]
    fn distribute_to_two_vehicles_with_uneven_stops() {
        let problem = Problem::new(
            vec![Vehicle::new(0, 0), Vehicle::new(4, 4)],
            vec![
                Stop::new(1),
                Stop::new(2),
                Stop::new(3),
                Stop::new(5),
                Stop::new(6),
            ],
            vec![
                Location::new(0.0, 0.0),
                Location::new(0.1, 0.0),
                Location::new(0.2, 0.0),
                Location::new(0.3, 0.0),
                Location::new(0.0, 1.0),
                Location::new(0.1, 1.0),
                Location::new(0.2, 1.0),
            ],
        );

        assert_eq!(
            solve(&problem),
            Solution::new(vec![vec![0, 1, 2].into(), vec![3, 4].into()])
        );
    }
}
