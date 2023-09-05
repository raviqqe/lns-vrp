use crate::{
    cost::{DeliveryCostCalculator, DistanceCostCalculator},
    problem::BaseProblem,
    route::CrowRouter,
    Location, SimpleProblem, Solution, Stop, Vehicle,
};
use rand::random;
use std::time::Instant;

const DISTANCE_COST: f64 = 1.0;
const MISSED_DELIVERY_COST: f64 = 1e9;

pub static ROUTER: CrowRouter = CrowRouter::new();

fn random_longitude() -> f64 {
    145.00647210413496 + 0.1 * random::<f64>()
}

fn random_latitude() -> f64 {
    -37.948738444529 + 0.1 * random::<f64>()
}

fn random_location() -> Location {
    Location::new(random_longitude(), random_latitude())
}

pub fn random_problem(vehicle_count: usize, stop_count: usize) -> SimpleProblem {
    SimpleProblem::new(
        (0..vehicle_count).map(|_| Vehicle::new()).collect(),
        (0..stop_count)
            .map(|_| Stop::new(random_location()))
            .collect(),
    )
}

pub fn create_cost_calculator(
    problem: &SimpleProblem,
) -> DeliveryCostCalculator<&CrowRouter, &SimpleProblem> {
    DeliveryCostCalculator::new(
        DistanceCostCalculator::new(&ROUTER, problem),
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
    println!("{}", solution.to_geojson(problem));
}
