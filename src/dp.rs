mod route;

use crate::Problem;
use core::{alloc::Allocator, hash::Hash};
use ordered_float::OrderedFloat;
use route::Route;
use std::collections::HashMap;

pub fn solve<'a, A: Allocator + Hash + Clone + 'a>(problem: &Problem) -> Option<Problem> {
    let mut states = HashMap::<Vec<Route>, f64>::new();
    let initial = problem.routes().map(|_| Route::new()).collect::<Vec<_>>();

    states.insert(initial, 0.0);

    for location in problem.routes().flat_map(crate::Route::stops) {
        let new_states = HashMap::new();

        for (routes, cost) in &states {
            for route in routes {
                todo!();
            }
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
                    .map(|route| crate::Route::new(route.stops().iter().cloned().collect()))
                    .collect(),
            )
        })
}
