mod distance;

use crate::Stop;
pub use distance::DistanceCostCalculator;

pub trait CostCalculator {
    fn calculate<'a>(routes: impl IntoIterator<Item = impl IntoIterator<Item = &'a Stop>>) -> f64;
}
