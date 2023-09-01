use super::solver::Solver;
use crate::{utility::calculate_route_cost, Problem, Stop};
use im_rc::{HashSet, Vector};
use ordered_float::OrderedFloat;

pub struct DynamicProgrammingSolver {}

impl DynamicProgrammingSolver {
    pub fn new() -> Self {
        Self {}
    }

    fn calculate_cost(&self, routes: &Vector<Vector<Stop>>) -> f64 {
        routes.iter().map(calculate_route_cost).sum()
    }
}

impl Solver for DynamicProgrammingSolver {
    fn solve(&self, problem: &Problem) -> Option<Problem> {
        let mut states = HashSet::<Vector<Vector<Stop>>>::new();
        let initial = problem
            .routes()
            .map(|_| Default::default())
            .collect::<Vector<_>>();

        states.insert(initial);

        let stop_count = problem.routes().flat_map(crate::Route::stops).count();

        for stop in problem.routes().flat_map(crate::Route::stops) {
            let mut new_states = HashSet::new();

            for routes in &states {
                for (index, stops) in routes.iter().enumerate() {
                    let mut stops = stops.clone();
                    stops.push_back(stop.clone());

                    let mut routes = routes.clone();
                    routes.set(index, stops);

                    // TODO Validate a route.
                    new_states.insert(routes);
                }
            }

            states = new_states;
        }

        states
        .iter()
        // TODO Validate routes in a general way.
        .filter(|routes| routes.iter().map(Vector::len).sum::<usize>() == stop_count)
        .min_by(|one, other| {
            OrderedFloat(self.calculate_cost(one)).cmp(&OrderedFloat(self.calculate_cost(other)))
        })
        .map(|routes| {
            Problem::new(
                routes
                    .iter()
                    .map(|stops| crate::Route::new(stops.iter().cloned().collect()))
                    .collect(),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Location, Route};

    fn solve(problem: &Problem) -> Option<Problem> {
        DynamicProgrammingSolver::new().solve(&problem)
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
