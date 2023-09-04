#[macro_export]
macro_rules! trace {
    ($expr:expr) => {
        #[cfg(feature = "trace")]
        dbg!($expr);
    };
}
