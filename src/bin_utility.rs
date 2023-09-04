use crate::{
    cost::{DeliveryCostCalculator, DistanceCostCalculator},
    problem::BaseProblem,
    Location, SimpleProblem, Solution, Stop, Vehicle,
};
use rand::random;
use std::time::Instant;

const STOP_COUNT: usize = 8;
const VEHICLE_COUNT: usize = 3;

const DISTANCE_COST: f64 = 1.0;
const MISSED_DELIVERY_COST: f64 = 1e9;

fn random_longitude() -> f64 {
    0.1 * random::<f64>()
}

fn random_location() -> Location {
    Location::new(random_longitude(), 0.0)
}

pub fn random_problem() -> SimpleProblem {
    SimpleProblem::new(
        (0..VEHICLE_COUNT).map(|_| Vehicle::new()).collect(),
        (0..STOP_COUNT)
            .map(|_| Stop::new(random_location()))
            .collect(),
    )
}

pub fn create_cost_calculator(problem: &SimpleProblem) -> DeliveryCostCalculator<&SimpleProblem> {
    DeliveryCostCalculator::new(
        DistanceCostCalculator::new(problem),
        problem.stops().len(),
        MISSED_DELIVERY_COST,
        DISTANCE_COST,
    )
}

pub fn measure_time<T>(callback: impl FnOnce() -> T) -> T {
    let instant = Instant::now();

    let value = callback();

    dbg!(Instant::now().duration_since(instant));

    value
}

pub fn print_solution(problem: impl BaseProblem, solution: &Solution) {
    println!(
        "{:#?}",
        solution
            .routes()
            .iter()
            .map(|indexes| indexes
                .iter()
                .map(|index| problem.stop_location(*index).as_point().x())
                .collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );
}