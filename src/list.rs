use alloc::boxed::Box;
use core::alloc::Allocator;

///  "Leaked" persistent linked list.
#[derive(Clone, Copy, Debug)]
pub struct List<'a, T, A: Allocator + 'a> {
    inner: Option<&'a Inner<'a, T>>,
    allocator: A,
}

#[derive(Debug)]
struct Inner<'a, T> {
    value: T,
    previous: Option<&'a Self>,
    next: Option<&'a Self>,
}

impl<'a, T, A: Allocator + Clone + 'a> List<'a, T, A> {
    pub fn new(allocator: A) -> Self {
        Self {
            inner: None,
            allocator,
        }
    }

    pub fn push_front(&self, value: T) -> Self {
        Self {
            inner: Some(self.create_inner(value, self.inner, None)),
            allocator: self.allocator.clone(),
        }
    }

    pub fn push_back(&self, value: T) -> Self {
        Self {
            inner: Some(self.create_inner(value, None, self.inner)),
            allocator: self.allocator.clone(),
        }
    }

    fn create_inner(
        &self,
        value: T,
        previous: Option<&'a Inner<'a, T>>,
        next: Option<&'a Inner<'a, T>>,
    ) -> &'a Inner<'a, T> {
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

impl<'a, T, A: Allocator + 'a> Iterator for List<'a, T, A> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(inner) = self.inner {
            self.inner = inner.next;
            return Some(&inner.value);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{alloc::Global, vec::Vec};

    #[test]
    fn create_empty() {
        assert_eq!(
            &List::<(), _>::new(Global).into_iter().collect::<Vec<_>>(),
            &[] as &[&()]
        );
    }

    #[test]
    fn push_back() {
        assert_eq!(
            &List::new(Global)
                .push_back(42)
                .into_iter()
                .collect::<Vec<_>>(),
            &[&42]
        );
    }

    #[test]
    fn push_back_two() {
        assert_eq!(
            &List::new(Global)
                .push_back(1)
                .push_back(2)
                .into_iter()
                .collect::<Vec<_>>(),
            &[&42]
        );
    }
}
