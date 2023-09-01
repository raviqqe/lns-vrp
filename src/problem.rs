use crate::Route;
use alloc::vec::Vec;
use core::alloc::Allocator;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Problem<A: Allocator + Clone> {
    routes: Vec<Route<A>, A>,
}

impl<A: Allocator + Clone> Problem<A> {
    pub fn new(routes: Vec<Route<A>, A>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &[Route<A>] {
        &self.routes
    }
}
