use crate::Route;
use alloc::vec::Vec;
use core::alloc::Allocator;

pub struct Problem<A: Allocator> {
    routes: Vec<Route<A>, A>,
}

impl<A: Allocator> Problem<A> {
    pub fn new(routes: Vec<Route<A>, A>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &[Route<A>] {
        &self.routes
    }
}
