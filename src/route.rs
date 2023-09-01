use crate::{List, Stop};
use core::alloc::Allocator;

#[derive(Clone, Debug)]
pub struct Route<'a, A: Allocator + 'a> {
    stops: List<'a, Stop, A>,
}

impl<'a, A: Allocator + 'a> Route<'a, A> {
    pub fn new(stops: List<'a, Stop, A>) -> Self {
        Self { stops }
    }

    pub fn stops(&self) -> impl Iterator<Item = &Stop> {
        self.stops.iter()
    }
}
