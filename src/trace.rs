#[macro_export]
macro_rules! trace {
    ($template:literal) => {
        trace!($template,);
    };
    ($template:literal, $($value:expr),*) => {
        #[cfg(feature = "trace")]
        println!($template, $($value),*);
    };
}
