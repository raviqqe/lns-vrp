mod delivery;

use crate::Solution;
pub use delivery::DeliveryCostCalculator;

pub trait CostCalculator {
    fn calculate(&self, solution: &Solution) -> f64;

    fn calculate_lower_bound(&self, solution: &Solution) -> f64 {
        self.calculate(solution)
    }
}
