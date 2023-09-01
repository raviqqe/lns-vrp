use alloc::alloc::Allocator;

///  "Leaked" persistent linked list.
pub struct List<'a, T, A: Allocator + 'a >{
    value: Option<&'a Inner<'a, T,  A>>
    allocator: A,
}

struct Inner<'a, T, A: Allocator + 'a> {
    value: T,
    previous: Option<&'a List<'a, T, A>>,
    next: Option<&'a List<'a, T>>,
}

impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn push_front(&self, value: T) -> Self {
        Self(Some(Inner {
            value,
            previous: Some(self),
            next: None,
        }))
    }

    pub fn push_back(&self, value: T) -> Self {
        Self(Some(Box::leak(Box::new_in(Inner {
            value,
            previous: Some(self),
            next: None,
        }))))
    }
}
