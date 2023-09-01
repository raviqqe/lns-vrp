mod route;

use crate::Problem;
use core::{
    alloc::Allocator,
    hash::{Hash, Hasher},
};
use ordered_float::OrderedFloat;
use route::Route;
use std::collections::{hash_map::DefaultHasher, HashMap};

pub fn solve<'a, A: Allocator + Hash + Clone + 'a>(problem: &Problem) -> Option<Problem> {
    // If hashes collide, let's have a party for it.
    let mut states = HashMap::<u64, (Vec<Route>, f64)>::new();
    let initial = problem.routes().map(|_| Route::new()).collect::<Vec<_>>();

    states.insert(hash(&initial), initial);

    let locations = problem.routes().flat_map(crate::Route::stops);

    for _state in &states {
        let _new_states = HashMap::<(), ()>::new();
    }

    states
        .values()
        .min_by(|(_, one), (_, other)| OrderedFloat(*one).cmp(&OrderedFloat(*other)))
        .map(|(routes, _)| {
            Problem::new(
                routes
                    .iter()
                    .map(|route| crate::Route::new(route.stops().iter().cloned().collect()))
                    .collect(),
            )
        })
}

fn hash(routes: &[Route]) -> u64 {
    let mut hasher = DefaultHasher::new();

    routes.hash(&mut hasher);

    hasher.finish()
}
