use crate::Problem;
use geo::GeodesicDistance;

#[derive(Debug)]
pub struct DistanceCostCalculator<'a> {
    problem: &'a Problem,
    cache: Vec<Vec<f64>>,
}

impl<'a> DistanceCostCalculator<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        let stop_count = problem.stops().len();

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

        let cost = self.problem.stops()[one]
            .location()
            .as_point()
            .geodesic_distance(self.problem.stops()[other].location().as_point());

        self.cache[one][other] = cost;

        cost
    }
}
