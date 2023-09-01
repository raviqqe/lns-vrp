pub struct List<'a, T>(Option<&'a Inner<'a, T>>);

struct Inner<'a, T> {
    value: T,
    previous: Option<&'a List<T>>,
    next: Option<&'a List<T>>,
}

impl List {
    pub fn new() -> Self {
        Self(None)
    }
}
