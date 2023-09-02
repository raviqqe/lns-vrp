use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use rand::random;
use vrp::{
    cost::DeliveryCostCalculator,
    solve::{BranchAndBoundSolver, DynamicProgrammingSolver, Solver},
    Location, Problem, Route, Stop,
};

const STOP_COUNT: usize = 11;
const VEHICLE_COUNT: usize = 2;

const DISTANCE_COST: f64 = 1.0;
const MISSED_DELIVERY_COST: f64 = 1e9;
const QUADRATIC_DISTANCE_COST: f64 = 1e-9;

fn random_longitude() -> f64 {
    0.1 * random::<f64>()
}

fn random_location() -> Location {
    Location::new(random_longitude(), 0.0)
}

fn random_problem() -> Problem {
    Problem::new(
        [Route::new(
            (0..STOP_COUNT)
                .map(|_| Stop::new(random_location()))
                .collect(),
        )]
        .into_iter()
        .chain((1..VEHICLE_COUNT).map(|_| Route::new(vec![])))
        .collect(),
    )
}

fn create_cost_calculator(problem: &Problem) -> DeliveryCostCalculator {
    DeliveryCostCalculator::new(
        problem.routes().flat_map(|route| route.stops()).count(),
        MISSED_DELIVERY_COST,
        DISTANCE_COST,
        QUADRATIC_DISTANCE_COST,
    )
}

fn dynamic_programming(bencher: &mut Bencher) {
    let problem = random_problem();
    let solver = DynamicProgrammingSolver::new(create_cost_calculator(&problem));

    bencher.iter(|| solver.solve(&problem).unwrap());
}

fn branch_and_bound(bencher: &mut Bencher) {
    let problem = random_problem();
    let solver = BranchAndBoundSolver::new(create_cost_calculator(&problem));

    bencher.iter(|| solver.solve(&problem).unwrap());
}

fn benchmark(criterion: &mut Criterion) {
    criterion.bench_function("dynamic programming", dynamic_programming);
    criterion.bench_function("branch and bound", branch_and_bound);
}

criterion_group!(benchmark_group, benchmark);
criterion_main!(benchmark_group);
