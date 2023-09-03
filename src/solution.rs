use alloc::vec::Vec;
use std::{
    alloc::{Allocator, Global},
    hash::{Hash, Hasher},
};

// TODO Use persistent data structure.
#[derive(Clone, Debug)]
pub struct Solution<A: Allocator = Global> {
    routes: Vec<Vec<usize, A>, A>,
}

impl<A: Allocator> Solution<A> {
    pub fn new(routes: Vec<Vec<usize, A>, A>) -> Self {
        Self { routes }
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

        Self { routes }
    }

    pub fn to_global(&self) -> Solution<Global> {
        Solution::new(self.routes().iter().map(|route| route.to_vec()).collect())
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
        for route in &self.routes {
            // Hash a boundary.
            false.hash(hasher);

            for index in route {
                index.hash(hasher);
            }
        }
    }
}
