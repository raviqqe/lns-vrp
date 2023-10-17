mod intermediate_route;

use self::intermediate_route::IntermediateRoute;
use alloc::vec::Vec;
use std::{
    alloc::{Allocator, Global},
    hash::Hash,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
