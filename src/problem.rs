use crate::Route;
use alloc::vec::Vec;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Problem {
    routes: Vec<Route>,
}

impl Problem {
    pub fn new(routes: Vec<Route>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &[Route] {
        &self.routes
    }
}
