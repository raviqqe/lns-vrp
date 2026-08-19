mod delivery;

use crate::Solution;
use allocator_api2::alloc::Allocator;
pub use delivery::DeliveryCostCalculator;

pub trait CostCalculator {
    fn calculate(&mut self, solution: &Solution<impl Allocator>) -> f64;

    fn calculate_lower_bound(&mut self, solution: &Solution<impl Allocator>) -> f64 {
        self.calculate(solution)
    }
}
