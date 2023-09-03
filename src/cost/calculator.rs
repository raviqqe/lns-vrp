mod delivery;
mod pickup_and_drop_off;

use crate::Solution;
pub use delivery::DeliveryCostCalculator;
pub use pickup_and_drop_off::PickupAndDropOffCostCalculator;
use std::alloc::Allocator;

pub trait CostCalculator {
    fn calculate(&mut self, solution: &Solution<impl Allocator>) -> f64;

    fn calculate_lower_bound(&mut self, solution: &Solution<impl Allocator>) -> f64 {
        self.calculate(solution)
    }
}
