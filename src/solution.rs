use alloc::vec::Vec;
use std::{
    alloc::{Allocator, Global},
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

// TODO Use persistent data structure.
// TODO Make it more compact.
#[derive(Clone, Debug)]
pub struct Solution<A: Allocator = Global> {
    hash: u64,
    routes: Vec<Vec<usize, A>, A>,
}

impl<A: Allocator> Solution<A> {
    pub fn new(routes: Vec<Vec<usize, A>, A>) -> Self {
        Self {
            hash: Self::hash(&routes),
            routes,
        }
    }

    pub fn routes(&self) -> &[Vec<usize, A>] {
        &self.routes
    }

    pub fn add_stop(&self, vehicle_index: usize, stop_index: usize) -> Self
    where
        A: Clone,
    {
        let mut route = self.routes[vehicle_index].clone();
        route.push(stop_index);

        let mut routes = self.routes.clone();
        routes[vehicle_index] = route;

        Self::new(routes)
    }

    pub fn to_global(&self) -> Solution<Global> {
        Solution::new(self.routes().iter().map(|route| route.to_vec()).collect())
    }

    fn hash(routes: &[Vec<usize, A>]) -> u64 {
        let mut hasher = DefaultHasher::new();
        routes.hash(&mut hasher);
        hasher.finish()
    }
}

impl<A: Allocator> Eq for Solution<A> {}

impl<A: Allocator> PartialEq for Solution<A> {
    fn eq(&self, other: &Self) -> bool {
        self.routes.len() == other.routes.len()
            && self.routes.iter().zip(&other.routes).all(|(one, other)| {
                one.len() == other.len() && one.iter().zip(other).all(|(one, other)| one == other)
            })
    }
}

impl<A: Allocator> Hash for Solution<A> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.hash.hash(hasher)
    }
}
