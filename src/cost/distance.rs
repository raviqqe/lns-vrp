use crate::Problem;
use geo::GeodesicDistance;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DistanceCostCalculator<'a> {
    problem: &'a Problem,
    cache: HashMap<(usize, usize), f64>,
}

impl<'a> DistanceCostCalculator<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        Self {
            problem,
            cache: Default::default(),
        }
    }

    pub fn calculate_route<'b>(&mut self, stop_indexes: &[usize]) -> f64 {
        let mut cost = 0.0;

        if let Some(mut previous_index) = stop_indexes.get(0).copied() {
            for &index in stop_indexes {
                cost += self.calculate_segment(previous_index, index);
                previous_index = index;
            }
        }

        cost
    }

    fn calculate_segment(&mut self, one: usize, other: usize) -> f64 {
        if let Some(&cost) = self.cache.get(&(one, other)) {
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
