use super::solver::Solver;
use crate::{cost::CostCalculator, Problem, Route, Stop};
use im_rc::{HashMap, Vector};
use ordered_float::OrderedFloat;

pub struct BranchAndBoundSolver<C: CostCalculator> {
    cost_calculator: C,
}

impl<C: CostCalculator> BranchAndBoundSolver<C> {
    pub fn new(cost_calculator: C) -> Self {
        Self { cost_calculator }
    }
}

impl<C: CostCalculator> Solver for BranchAndBoundSolver<C> {
    fn solve(&self, problem: &Problem) -> Option<Problem> {
        let mut states = HashMap::<Vector<Vector<Stop>>, f64>::new();
        let routes = problem
            .routes()
            .map(|_| Default::default())
            .collect::<Vector<_>>();

        let cost = self.cost_calculator.calculate(&routes);
        states.insert(routes, cost);

        for stop in problem.routes().flat_map(Route::stops) {
            let mut new_states = HashMap::new();

            for (initial_routes, cost) in &states {
                let mut routes = initial_routes.clone();
                let mut cost = *cost;

                for (index, stops) in initial_routes.iter().enumerate() {
                    let new_routes = {
                        let mut stops = stops.clone();
                        stops.push_back(stop.clone());

                        let mut routes = initial_routes.clone();
                        routes.set(index, stops);
                        routes
                    };
                    let new_cost = self.cost_calculator.calculate(&new_routes);

                    // TODO Check finity of a cost?

                    if new_cost < cost {
                        cost = new_cost;
                        routes = new_routes;
                    }
                }

                new_states.insert(routes, cost);
            }

            states = new_states;
        }

        states
            .iter()
            .min_by(|(_, &one), (_, &other)| OrderedFloat(one).cmp(&OrderedFloat(other)))
            .map(|(routes, _)| {
                Problem::new(
                    routes
                        .iter()
                        .map(|stops| Route::new(stops.iter().cloned().collect()))
                        .collect(),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cost::DeliveryCostCalculator, Location, Route};

    const DISTANCE_COST: f64 = 1.0;
    const MISSED_DELIVERY_COST: f64 = 1e9;
    const QUADRATIC_DISTANCE_COST: f64 = 1e-9;

    fn solve(problem: &Problem) -> Option<Problem> {
        BranchAndBoundSolver::new(DeliveryCostCalculator::new(
            problem.routes().flat_map(|route| route.stops()).count(),
            MISSED_DELIVERY_COST,
            DISTANCE_COST,
            QUADRATIC_DISTANCE_COST,
        ))
        .solve(problem)
    }

    #[test]
    fn do_nothing() {
        let problem = Problem::new(vec![]);

        assert_eq!(solve(&problem), Some(problem));
    }

    #[test]
    fn keep_one_stop() {
        let problem = Problem::new(vec![Route::new(vec![Stop::new(Location::new(0.0, 0.0))])]);

        assert_eq!(solve(&problem), Some(problem));
    }

    #[test]
    fn keep_two_stops() {
        let problem = Problem::new(vec![Route::new(vec![
            Stop::new(Location::new(0.0, 0.0)),
            Stop::new(Location::new(1.0, 0.0)),
        ])]);

        assert_eq!(solve(&problem), Some(problem));
    }

    #[test]
    fn keep_three_stops() {
        let problem = Problem::new(vec![Route::new(vec![
            Stop::new(Location::new(0.0, 0.0)),
            Stop::new(Location::new(1.0, 0.0)),
            Stop::new(Location::new(2.0, 0.0)),
        ])]);

        assert_eq!(solve(&problem), Some(problem));
    }

    #[test]
    fn even_workload() {
        let problem = Problem::new(vec![
            Route::new(vec![
                Stop::new(Location::new(0.0, 0.0)),
                Stop::new(Location::new(1.0, 0.0)),
                Stop::new(Location::new(2.0, 0.0)),
            ]),
            Route::new(vec![]),
        ]);

        assert!(solve(&problem)
            .unwrap()
            .routes()
            .all(|route| route.stops().len() < 3));
    }
}
