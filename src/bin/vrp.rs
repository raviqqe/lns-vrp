use rand::random;
use vrp::{
    cost::DeliveryCostCalculator,
    solve::{DynamicProgrammingSolver, Solver},
    Location, Problem, Route, Stop,
};

const STOP_COUNT: usize = 10;
const VEHICLE_COUNT: usize = 3;

const DISTANCE_COST: f64 = 1.0;
const MISSED_DELIVERY_COST: f64 = 1e9;
const QUADRATIC_DISTANCE_COST: f64 = 1e-9;

fn random_longitude() -> f64 {
    0.1 * random::<f64>()
}

fn random_location() -> Location {
    Location::new(random_longitude(), 0.0)
}

fn main() {
    let problem = Problem::new(
        [Route::new(
            (0..STOP_COUNT)
                .map(|_| Stop::new(random_location()))
                .collect(),
        )]
        .into_iter()
        .chain((1..VEHICLE_COUNT).map(|_| Route::new(vec![])))
        .collect(),
    );

    let solver = DynamicProgrammingSolver::new(DeliveryCostCalculator::new(
        problem.routes().flat_map(|route| route.stops()).count(),
        MISSED_DELIVERY_COST,
        DISTANCE_COST,
        QUADRATIC_DISTANCE_COST,
    ));

    solver.solve(&problem).unwrap();
}
