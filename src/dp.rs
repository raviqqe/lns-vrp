mod route;

use crate::Problem;
use core::{
    alloc::Allocator,
    hash::{Hash, Hasher},
};
use route::Route;
use std::collections::{hash_map::DefaultHasher, HashMap};

pub fn solve<'a, A: Allocator + Hash + Clone + 'a>(problem: &Problem) -> Option<Problem> {
    // If hashes collide, let's have a party for it.
    let mut states = HashMap::<u64, Vec<Route>>::new();
    let initial = problem.routes().map(|_| Route::new()).collect::<Vec<_>>();

    states.insert(hash(&initial), initial);

    let locations = problem.routes().flat_map(crate::Route::stops);

    states.insert(hash(problem), problem.clone());

    for _state in &states {
        let _new_states = HashMap::<(), ()>::new();
    }

    None
}

fn hash(routes: &[Route]) -> u64 {
    let mut hasher = DefaultHasher::new();

    routes.hash(&mut hasher);

    hasher.finish()
}
