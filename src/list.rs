///  "Leaked" persistent linked list.
pub struct List<'a, T>(Option<&'a Inner<'a, T>>);

struct Inner<'a, T> {
    value: T,
    previous: Option<&'a List<'a, T>>,
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
}
