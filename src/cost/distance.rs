use crate::{problem::BaseProblem, route::Router};

// TODO Should this be agnostic about problems?
#[derive(Debug)]
pub struct DistanceCostCalculator<R: Router, P: BaseProblem> {
    router: R,
    problem: P,
    cache: Vec<Vec<f64>>,
}

impl<R: Router, P: BaseProblem> DistanceCostCalculator<R, P> {
    pub fn new(router: R, problem: P) -> Self {
        let stop_count = problem.stop_count();

        Self {
            router,
            problem,
            cache: vec![vec![f64::NAN; stop_count]; stop_count],
        }
    }

    pub fn calculate_route(&mut self, vehicle_index: usize, stop_indexes: &[usize]) -> f64 {
        if let (Some(&first_stop_index), Some(&last_stop_index)) =
            (stop_indexes.first(), stop_indexes.last())
        {
            self.router.route(
                self.problem.vehicle_start_location(vehicle_index),
                self.problem.stop_location(first_stop_index),
            ) + stop_indexes
                .iter()
                .zip(stop_indexes.iter().skip(1))
                .map(|(&one, &other)| self.calculate_segment(one, other))
                .sum::<f64>()
                + self.router.route(
                    self.problem.stop_location(last_stop_index),
                    self.problem.vehicle_end_location(vehicle_index),
                )
        } else {
            self.router.route(
                self.problem.vehicle_start_location(vehicle_index),
                self.problem.vehicle_end_location(vehicle_index),
            )
        }
    }

    fn calculate_segment(&mut self, one: usize, other: usize) -> f64 {
        let cached = self.cache[one][other];

        if !cached.is_nan() {
            return cached;
        }

        let cost = self.router.route(
            self.problem.stop_location(one),
            self.problem.stop_location(other),
        );

        self.cache[one][other] = cost;

        cost
    }
}
