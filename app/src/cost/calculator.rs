mod delivery;

use core::BasicSolution;
pub use delivery::DeliveryCostCalculator;
use std::alloc::Allocator;

pub trait CostCalculator {
    fn calculate(&mut self, solution: &BasicSolution<impl Allocator>) -> f64;

    fn calculate_lower_bound(&mut self, solution: &BasicSolution<impl Allocator>) -> f64 {
        self.calculate(solution)
    }
}
