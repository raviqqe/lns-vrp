use crate::Problem;
use geo::GeodesicDistance;

#[derive(Debug)]
pub struct DistanceCostCalculator<'a> {
    problem: &'a Problem,
    cache: Vec<f64>,
}

impl<'a> DistanceCostCalculator<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        let stop_count = problem.stops().len();

        Self {
            problem,
            cache: vec![f64::NAN; stop_count * stop_count],
        }
    }

    pub fn calculate_route(&mut self, stop_indexes: &[usize]) -> f64 {
        let mut cost = 0.0;

        if let Some(mut previous_index) = stop_indexes.first().copied() {
            for &index in stop_indexes {
                cost += self.calculate_segment(previous_index, index);
                previous_index = index;
            }
        }

        cost
    }

    fn calculate_segment(&mut self, one: usize, other: usize) -> f64 {
        if let Some(&cost) = self.cache.get([one * problem.stop_count() + other]) {
            return cost;
        }

        let cost = self.problem.stops()[one]
            .location()
            .as_point()
            .geodesic_distance(self.problem.stops()[other].location().as_point());

        self.cache.insert((one, other), cost);

        cost
    }
}
