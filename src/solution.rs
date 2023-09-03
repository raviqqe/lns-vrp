use crate::Stop;
use alloc::vec::Vec;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Solution {
    routes: Vec<Vec<usize>>,
}

impl Solution {
    pub fn new(routes: Vec<Vec<usize>>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &[Vec<usize>] {
        &self.routes
    }
}
