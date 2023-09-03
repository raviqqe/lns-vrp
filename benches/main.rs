use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use rand::random;
use vrp::{
    cost::{DeliveryCostCalculator, DistanceCostCalculator},
    solve::{BranchAndBoundSolver, DynamicProgrammingSolver, Solver},
    Location, Problem, Stop, Vehicle,
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
        (0..VEHICLE_COUNT).map(|_| Vehicle::new()).collect(),
        (0..STOP_COUNT)
            .map(|_| Stop::new(random_location()))
            .collect(),
    )
}

fn create_cost_calculator(problem: &Problem) -> DeliveryCostCalculator {
    DeliveryCostCalculator::new(
        DistanceCostCalculator::new(problem),
        problem.stops().len(),
        MISSED_DELIVERY_COST,
        DISTANCE_COST,
        QUADRATIC_DISTANCE_COST,
    )
}

fn dynamic_programming(bencher: &mut Bencher) {
    let problem = random_problem();
    let solver = DynamicProgrammingSolver::new(create_cost_calculator(&problem));

    bencher.iter(|| solver.solve(&problem));
}

fn branch_and_bound(bencher: &mut Bencher) {
    let problem = random_problem();
    let solver = BranchAndBoundSolver::new(create_cost_calculator(&problem));

    bencher.iter(|| solver.solve(&problem));
}

fn benchmark(criterion: &mut Criterion) {
    criterion.bench_function("dynamic programming", dynamic_programming);
    criterion.bench_function("branch and bound", branch_and_bound);
}

criterion_group!(benchmark_group, benchmark);
criterion_main!(benchmark_group);
