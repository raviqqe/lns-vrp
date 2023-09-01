use alloc::boxed::Box;
use core::{alloc::Allocator, fmt::Debug, ptr::write};

///  "Leaked" persistent linked list.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct List<'a, T, A: Allocator + 'a> {
    first: Option<&'a Inner<'a, T>>,
    last: Option<&'a Inner<'a, T>>,
    allocator: A,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Inner<'a, T> {
    value: T,
    previous: Option<&'a Self>,
    next: Option<&'a Self>,
}

impl<'a, T, A: Allocator + Clone + 'a> List<'a, T, A> {
    pub fn new(allocator: A) -> Self {
        Self {
            first: None,
            last: None,
            allocator,
        }
    }

    pub fn push_front(&self, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            first: Some(if let Some(first) = self.first {
                let next = self.clone_inner(first);
                let first = self.create_inner(value, None, None);

                unsafe { Self::link(first, next) };

                first
            } else {
                self.create_inner(value, None, None)
            }),
            last: self.last,
            allocator: self.allocator.clone(),
        }
    }

    pub fn push_back(&self, value: T) -> Self
    where
        T: Clone + Debug,
    {
        if let Some(last) = self.last {
            let previous = self.clone_inner(last);
            let last = self.create_inner(value, None, None);

            unsafe { Self::link(previous, last) };

            Self {
                first: if self.is_singleton() {
                    Some(previous)
                } else {
                    self.first
                },
                last: Some(last),
                allocator: self.allocator.clone(),
            }
        } else {
            let last = Some(&*self.create_inner(value, None, None));

            Self {
                first: last,
                last,
                allocator: self.allocator.clone(),
            }
        }
    }

    fn is_singleton(&self) -> bool {
        if let (Some(first), Some(last)) = (self.first, self.last) {
            first as *const _ == last
        } else {
            false
        }
    }

    /// # Safety
    ///
    /// Two nodes must be newly allocated by a list's allocator.
    unsafe fn link(one: *mut Inner<'a, T>, other: *mut Inner<'a, T>) {
        write(&mut (*one).next, Some(&*other));
        write(&mut (*other).previous, Some(&*one));
    }

    fn create_inner(
        &self,
        value: T,
        previous: Option<&'a Inner<'a, T>>,
        next: Option<&'a Inner<'a, T>>,
    ) -> &'a mut Inner<'a, T> {
        Box::leak(Box::new_in(
            Inner {
                value,
                previous,
                next,
            },
            self.allocator.clone(),
        ))
    }

    fn clone_inner(&self, inner: &'a Inner<'a, T>) -> &'a mut Inner<'a, T>
    where
        T: Clone,
    {
        Box::leak(Box::new_in(inner.clone(), self.allocator.clone()))
    }
}

pub struct Iter<'a, T> {
    inner: Option<&'a Inner<'a, T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(inner) = self.inner {
            self.inner = inner.next;
            return Some(&inner.value);
        }

        None
    }
}

impl<'a, T, A: Allocator + 'a> IntoIterator for &List<'a, T, A> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { inner: self.first }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{alloc::Global, vec::Vec};
    use pretty_assertions::assert_eq;

    #[test]
    fn create_empty() {
        assert_eq!(
            &List::<(), _>::new(Global).into_iter().collect::<Vec<_>>(),
            &[] as &[&()]
        );
    }

    mod push_front {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn one() {
            assert_eq!(
                &List::new(Global)
                    .push_front(1)
                    .into_iter()
                    .collect::<Vec<_>>(),
                &[&1]
            );
        }

        #[test]
        fn two() {
            assert_eq!(
                &List::new(Global)
                    .push_front(1)
                    .push_front(2)
                    .into_iter()
                    .collect::<Vec<_>>(),
                &[&2, &1]
            );
        }

        #[test]
        fn three() {
            assert_eq!(
                &List::new(Global)
                    .push_front(1)
                    .push_front(2)
                    .push_front(3)
                    .into_iter()
                    .collect::<Vec<_>>(),
                &[&3, &2, &1]
            );
        }
    }

    mod push_back {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn one() {
            assert_eq!(
                &List::new(Global)
                    .push_back(1)
                    .into_iter()
                    .collect::<Vec<_>>(),
                &[&1]
            );
        }

        #[test]
        fn two() {
            assert_eq!(
                &List::new(Global)
                    .push_back(1)
                    .push_back(2)
                    .into_iter()
                    .collect::<Vec<_>>(),
                &[&1, &2]
            );
        }

        #[test]
        fn three() {
            assert_eq!(
                &List::new(Global)
                    .push_back(1)
                    .push_back(2)
                    .push_back(3)
                    .into_iter()
                    .collect::<Vec<_>>(),
                &[&1, &2, &3]
            );
        }
    }
}
