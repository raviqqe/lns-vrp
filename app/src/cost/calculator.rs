mod delivery;

use core::Solution;
pub use delivery::DeliveryCostCalculator;
use std::alloc::Allocator;

pub trait CostCalculator {
    fn calculate(&mut self, solution: &Solution<impl Allocator>) -> f64;

    fn calculate_lower_bound(&mut self, solution: &Solution<impl Allocator>) -> f64 {
        self.calculate(solution)
    }
}
