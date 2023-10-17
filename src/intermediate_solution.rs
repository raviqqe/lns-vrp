mod intermediate_route;

pub(crate) use self::intermediate_route::IntermediateRoute;
use alloc::vec::Vec;
use std::{
    alloc::{Allocator, Global},
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug)]
pub struct IntermediateSolution<A: Allocator = Global> {
    routes: Vec<IntermediateRoute, A>,
}

impl<A: Allocator> IntermediateSolution<A> {
    pub fn new(routes: Vec<IntermediateRoute, A>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &[IntermediateRoute] {
        &self.routes
    }
}

impl<A: Allocator> Eq for IntermediateSolution<A> {}

impl<A: Allocator> PartialEq for IntermediateSolution<A> {
    fn eq(&self, other: &Self) -> bool {
        self.routes == other.routes
    }
}

impl<A: Allocator> Hash for IntermediateSolution<A> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.routes.hash(hasher)
    }
}
