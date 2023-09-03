mod delivery;

use crate::Stop;
pub use delivery::DeliveryCostCalculator;

pub trait CostCalculator {
    fn calculate(&self, solution: &Solution) -> f64;

    fn calculate_lower_bound<'a>(&self, solution: &Solution) -> f64 {
        self.calculate(solution)
    }
}
