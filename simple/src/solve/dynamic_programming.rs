use crate::{cost::CostCalculator, hash_map::HashMap, Problem, Solution, Stop, Vehicle};
use bumpalo::Bump;
use core::{BasicProblem, BasicSolver};
use ordered_float::OrderedFloat;
use std::alloc::Global;

/// Dynamic programming solver.
pub struct DynamicProgrammingSolver<C: CostCalculator> {
    cost_calculator: C,
}

impl<C: CostCalculator> DynamicProgrammingSolver<C> {
    pub fn new(cost_calculator: C) -> Self {
        Self { cost_calculator }
    }
}

impl<C: CostCalculator> BasicSolver<Vehicle, Stop, Problem, Solution>
    for DynamicProgrammingSolver<C>
{
    fn solve(&mut self, problem: &Problem) -> Solution {
        let stop_count = problem.stop_count();
        let vehicle_count = problem.vehicle_count();
        let mut dp = vec![vec![vec![f64::INFINITY; stop_count]; vehicle_count]; 1 << stop_count];

        for index in 0..stop_count {
            dp[0][0][index] = 0.0;
        }

        for stop_set in 0..1 << stop_count {
            for vehicle_index in 0..vehicle_count {
                for stop_index in 0..stop_count {
                    if dp[stop_set][vehicle_index][stop_index].is_infinite() {
                        continue;
                    }

                    for next_stop_index in 0..stop_count {
                        if 1 << next_stop_index & stop_set > 0 {
                            continue;
                        }

                        let next_stop_set = stop_set | 1 << next_stop_index;

                        dp[next_stop_set][vehicle_index][next_stop_index] =
                            dp[next_stop_set][vehicle_index][next_stop_index].min(
                                dp[stop_set][vehicle_index][stop_index]
                                    + distance(stop_index, next_stop_index, xs),
                            );

                        if vehicle_index + 1 < vehicle_count {
                            for (ii, kk) in
                                [(stop_set, stop_index), (next_stop_set, next_stop_index)]
                            {
                                dp[ii][vehicle_index + 1][kk] = dp[ii][vehicle_index + 1][kk]
                                    .min(dp[stop_set][vehicle_index][stop_index]);
                            }
                        }
                    }
                }
            }
        }

        let value = *dp
            .last()
            .unwrap()
            .last()
            .unwrap()
            .iter()
            .min_by_key(|&&x| OrderedFloat(x))
            .unwrap();

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cost::{DeliveryCostCalculator, DistanceCostCalculator},
        route::CrowRouter,
        Problem, Stop, Vehicle,
    };
    use core::Location;

    const DISTANCE_COST: f64 = 1.0;
    const QUADRATIC_DISTANCE_COST: f64 = 1e-3;
    const MISSED_DELIVERY_COST: f64 = 1e9;

    static ROUTER: CrowRouter = CrowRouter::new();

    fn solve(problem: &Problem) -> Solution {
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
        let problem = Problem::new(
            vec![Vehicle::new(0, 1)],
            vec![],
            vec![Location::new(0.0, 0.0), Location::new(1.0, 0.0)],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![].into()]));
    }

    #[test]
    fn keep_one_stop() {
        let problem = Problem::new(
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
        let problem = Problem::new(
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

        assert_eq!(solve(&problem), Solution::new(vec![vec![0, 1, 2].into()]));
    }

    #[test]
    fn even_workload() {
        let problem = Problem::new(
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
