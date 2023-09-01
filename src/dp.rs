use crate::{utility::calculate_route_cost, Problem, Stop};
use im::{HashSet, Vector};
use ordered_float::OrderedFloat;

pub fn solve(problem: &Problem) -> Option<Problem> {
    let mut states = HashSet::<Vector<Vector<Stop>>>::new();
    let initial = problem
        .routes()
        .map(|_| Default::default())
        .collect::<Vector<_>>();

    states.insert(initial);

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
        .min_by(|one, other| {
            OrderedFloat(calculate_cost(one)).cmp(&OrderedFloat(calculate_cost(other)))
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

fn calculate_cost(routes: &Vector<Vector<Stop>>) -> f64 {
    routes.iter().map(calculate_route_cost).sum()
}
