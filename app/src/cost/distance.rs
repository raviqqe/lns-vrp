use crate::route::Router;
use core::BasicProblem;
use std::cell::RefCell;

#[derive(Debug)]
pub struct DistanceCostCalculator<R: Router, P: BasicProblem> {
    router: R,
    problem: P,
    cache: RefCell<Vec<Vec<f64>>>,
}

impl<R: Router, P: BasicProblem> DistanceCostCalculator<R, P> {
    pub fn new(router: R, problem: P) -> Self {
        let location_count = problem.location_count();

        Self {
            router,
            problem,
            cache: vec![vec![f64::NAN; location_count]; location_count].into(),
        }
    }

    pub fn calculate_route(&self, vehicle_index: usize, stop_indexes: &[usize]) -> f64 {
        [self.problem.vehicle_start_location(vehicle_index)]
            .into_iter()
            .chain(
                stop_indexes
                    .iter()
                    .map(|&index| self.problem.stop_location(index)),
            )
            .zip(
                stop_indexes
                    .iter()
                    .map(|&index| self.problem.stop_location(index))
                    .chain([self.problem.vehicle_end_location(vehicle_index)]),
            )
            .map(|(one, other)| self.calculate_segment(one, other))
            .sum::<f64>()
    }

    fn calculate_segment(&self, one: usize, other: usize) -> f64 {
        let cached = self.cache.borrow()[one][other];

        if !cached.is_nan() {
            return cached;
        }

        let cost = self
            .router
            .route(self.problem.location(one), self.problem.location(other));

        self.cache.borrow_mut()[one][other] = cost;

        cost
    }
}
