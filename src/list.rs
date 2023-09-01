pub struct List<T>(&'static Inner<T>);

struct Inner<T> {
    value: T,
    previous: Option<&'static List<T>>,
    next: Option<&'static List<T>>,
}
