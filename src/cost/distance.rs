use crate::problem::BaseProblem;
use geo::GeodesicDistance;

#[derive(Debug)]
pub struct DistanceCostCalculator<P: BaseProblem> {
    problem: P,
    cache: Vec<Vec<f64>>,
}

impl<P: BaseProblem> DistanceCostCalculator<P> {
    pub fn new(problem: P) -> Self {
        let stop_count = problem.stop_count();

        Self {
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

        let cost = self
            .problem
            .stop_location(one)
            .as_point()
            .geodesic_distance(self.problem.stop_location(other).as_point());

        self.cache[one][other] = cost;

        cost
    }
}
