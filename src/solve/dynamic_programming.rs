use super::solver::Solver;
use crate::{cost::CostCalculator, Problem, Route, Stop};
use im_rc::{HashSet, Vector};
use ordered_float::OrderedFloat;

/// Dyanmic programming solver.
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
    fn solve(&self, problem: &Problem) -> Option<Problem> {
        let mut states = HashSet::<Vector<Vector<Stop>>>::new();
        let initial = problem
            .routes()
            .map(|_| Default::default())
            .collect::<Vector<_>>();

        states.insert(initial);

        for stop in problem.routes().flat_map(Route::stops) {
            let mut new_states = states.clone();

            for routes in &states {
                for (index, stops) in routes.iter().enumerate() {
                    let mut stops = stops.clone();
                    stops.push_back(stop.clone());

                    let mut routes = routes.clone();
                    routes.set(index, stops);

                    if self.cost_calculator.calculate(&routes).is_finite() {
                        new_states.insert(routes);
                    }
                }
            }

            states = new_states;
        }

        states
            .iter()
            .map(|routes| (routes, self.cost_calculator.calculate(routes)))
            .min_by(|(_, one), (_, other)| OrderedFloat(*one).cmp(&OrderedFloat(*other)))
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

    const DISTANCE_UNIT_COST: f64 = 1.0;
    const MISSED_DELIVERY_COST: f64 = 1e9;

    fn solve(problem: &Problem) -> Option<Problem> {
        DynamicProgrammingSolver::new(DeliveryCostCalculator::new(
            problem.routes().flat_map(|route| route.stops()).count(),
            MISSED_DELIVERY_COST,
            DISTANCE_UNIT_COST,
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
}
