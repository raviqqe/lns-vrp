use alloc::{alloc::Allocator, boxed::Box};

///  "Leaked" persistent linked list.
#[derive(Clone, Copy, Debug)]
pub struct List<'a, T, A: Allocator + 'a> {
    value: Option<&'a Inner<'a, T, A>>,
    allocator: A,
}

#[derive(Debug)]
struct Inner<'a, T, A: Allocator + 'a> {
    value: T,
    previous: Option<&'a List<'a, T, A>>,
    next: Option<&'a List<'a, T, A>>,
}

impl<'a, T, A: Allocator + Clone + 'a> List<'a, T, A> {
    pub fn new(allocator: A) -> Self {
        Self {
            value: None,
            allocator,
        }
    }

    pub fn push_front(&self, value: T) -> Self {
        Self {
            value: Some(self.create_inner(value, Some(self), None)),
            allocator: self.allocator.clone(),
        }
    }

    pub fn push_back(&self, value: T) -> Self {
        Self {
            value: Some(self.create_inner(value, None, Some(self))),
            allocator: self.allocator.clone(),
        }
    }

    fn create_inner(
        &self,
        value: T,
        previous: Option<&'a List<'a, T, A>>,
        next: Option<&'a List<'a, T, A>>,
    ) -> &'a Inner<'a, T, A> {
        Box::leak(Box::new_in(
            Inner {
                value,
                previous,
                next,
            },
            self.allocator.clone(),
        ))
    }
}
