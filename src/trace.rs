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

#[macro_export]
macro_rules! trace_solution {
    ($solution:expr, $cost:expr) => {
        trace!("new solution found!");
        trace!("solution: {:?}", $solution);
        trace!("cost: {:?}", $cost);
    };
}
