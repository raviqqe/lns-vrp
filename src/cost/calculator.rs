mod delivery;
mod distance;

use crate::Stop;
pub use delivery::DeliveryCostCalculator;
pub use distance::DistanceCostCalculator;

pub trait CostCalculator {
    fn calculate<'a>(
        &self,
        routes: impl IntoIterator<
            Item = impl IntoIterator<
                Item = &'a Stop,
                IntoIter = impl ExactSizeIterator<Item = &'a Stop>,
            >,
        >,
    ) -> f64;
}
