use crate::{Problem, Route};
use core::{
    alloc::Allocator,
    hash::{Hash, Hasher},
};
use std::collections::{hash_map::DefaultHasher, HashMap};

pub fn solve<'a, A: Allocator + Hash + Clone + 'a>(problem: &Problem<A>) -> Option<Problem<A>> {
    let mut states = HashMap::<u64, Problem<_>>::new();
    let _locations = problem.routes().iter().flat_map(Route::stops);

    states.insert(hash(problem), problem.clone());

    for _state in &states {
        let _new_states = HashMap::<(), ()>::new();
    }

    None
}

fn hash<A: Allocator + Hash + Clone>(problem: &Problem<A>) -> u64 {
    let mut hasher = DefaultHasher::new();

    problem.hash(&mut hasher);

    hasher.finish()
}
