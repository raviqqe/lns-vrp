use crate::{problem::BaseProblem, route::Router};

// TODO Should this be agnositc about problems?
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

    pub fn calculate_route(&mut self, stop_indexes: &[usize]) -> f64 {
        stop_indexes
            .iter()
            .zip(stop_indexes.iter().skip(1))
            .map(|(&one, &other)| self.calculate_segment(one, other))
            .sum()
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
