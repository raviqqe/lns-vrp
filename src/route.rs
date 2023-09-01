use crate::Stop;
use alloc::collections::LinkedList;
use core::alloc::Allocator;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Route<A: Allocator> {
    stops: LinkedList<Stop, A>,
}

impl<A: Allocator> Route<A> {
    pub fn new(stops: LinkedList<Stop, A>) -> Self {
        Self { stops }
    }

    pub fn stops(&self) -> impl Iterator<Item = &Stop> {
        self.stops.iter()
    }
}
