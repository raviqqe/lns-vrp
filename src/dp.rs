use crate::{Context, Problem, Route};
use core::{
    alloc::Allocator,
    hash::{Hash, Hasher},
};
use std::collections::{hash_map::DefaultHasher, HashMap};

pub fn solve<'a, A: Allocator + Hash + Clone + 'a>(
    context: &Context<A>,
    problem: &Problem<A>,
) -> bool {
    let mut states = HashMap::<u64, Problem<_>>::new();
    let locations = problem.routes().iter().flat_map(Route::stops);

    states.insert(hash(problem), problem.clone());

    for state in &states {
        let _new_states = HashMap::<(), ()>::new();
    }

    !states.is_empty()
}

fn hash<A: Allocator + Hash + Clone>(problem: &Problem<A>) -> u64 {
    let mut hasher = DefaultHasher::new();

    problem.hash(&mut hasher);

    hasher.finish()
}
