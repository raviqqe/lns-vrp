#[macro_export]
macro_rules! trace {
    ($template:literal) => {
        trace!($template,);
    };
    ($template:literal, $($expr:expr),*) => {
        #[cfg(feature = "trace")]
        println!($template, $($expr),*);
    };
}
