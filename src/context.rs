use core::alloc::Allocator;

pub struct Context<A: Allocator> {
    allocator: A,
}

impl<A: Allocator> Context<A>
where
    for<'a> &'a A: Allocator,
{
    pub fn new(allocator: A) -> Self {
        Self { allocator }
    }

    pub fn allocator(&self) -> &A {
        &self.allocator
    }
}
