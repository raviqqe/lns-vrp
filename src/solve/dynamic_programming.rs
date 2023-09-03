use super::solver::Solver;
use crate::{cost::CostCalculator, Problem, Route, Solution, Stop};
use im_rc::{HashSet, Vector};
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
    fn solve(&self, problem: &Problem) -> Solution {
        let mut solutions = HashSet::<Vector<Vector<Stop>>>::new();

        solutions.insert(
            problem
                .vehicles()
                .map(|_| Default::default())
                .collect::<Vector<_>>(),
        );

        for stop in problem.stops() {
            let mut new_solutions = solutions.clone();

            for routes in &solutions {
                for (index, stops) in routes.iter().enumerate() {
                    let mut stops = stops.clone();
                    stops.push_back(stop.clone());

                    let mut routes = routes.clone();
                    routes.set(index, stops);

                    if self.cost_calculator.calculate(&routes).is_finite() {
                        new_solutions.insert(routes);
                    }
                }
            }

            solutions = new_solutions;
        }

        Solution::new(
            solutions
                .iter()
                .map(|routes| (routes, self.cost_calculator.calculate(routes)))
                .min_by(|(_, one), (_, other)| OrderedFloat(*one).cmp(&OrderedFloat(*other)))
                .expect("at least one solution")
                .0
                .iter()
                .map(|stops| stops.iter().copied().collect())
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cost::DeliveryCostCalculator, Location, Route};

    const DISTANCE_COST: f64 = 1.0;
    const MISSED_DELIVERY_COST: f64 = 1e9;
    const QUADRATIC_DISTANCE_COST: f64 = 1e-9;

    fn solve(problem: &Problem) -> Solution {
        DynamicProgrammingSolver::new(DeliveryCostCalculator::new(
            problem.stops().len(),
            MISSED_DELIVERY_COST,
            DISTANCE_COST,
            QUADRATIC_DISTANCE_COST,
        ))
        .solve(problem)
    }

    #[test]
    fn do_nothing() {
        let problem = Problem::new(vec![], vec![]);

        assert_eq!(solve(&problem), Solution::new(vec![vec![]]));
    }

    #[test]
    fn keep_one_stop() {
        let problem = Problem::new(
            vec![Vehicle::new()],
            vec![Stop::new(Location::new(0.0, 0.0))],
        );

        assert_eq!(solve(&problem), Solution::new(vec![vec![0]]));
    }

    #[test]
    fn keep_two_stops() {
        let problem = Problem::new(
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
        let problem = Problem::new(
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
        let problem = Problem::new(
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
