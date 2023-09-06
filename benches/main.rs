use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use rand::random;
use vrp::{
    cost::{DeliveryCostCalculator, DistanceCostCalculator},
    route::CrowRouter,
    solve::{
        BranchAndBoundSolver, DynamicProgrammingSolver, NearestNeighborSolver,
        RuinAndRecreateSolver, Solver,
    },
    Location, SimpleProblem, Stop, Vehicle,
};

const STOP_COUNT: usize = 8;
const VEHICLE_COUNT: usize = 2;
const ITERATION_COUNT: usize = 100;

const DISTANCE_COST: f64 = 1.0;
const MISSED_DELIVERY_COST: f64 = 1e9;

static ROUTER: CrowRouter = CrowRouter::new();

fn random_longitude() -> f64 {
    0.1 * random::<f64>()
}

fn random_location() -> Location {
    Location::new(random_longitude(), 0.0)
}

fn random_problem() -> SimpleProblem {
    SimpleProblem::new(
        (0..VEHICLE_COUNT)
            .map(|_| Vehicle::new(random_location(), random_location()))
            .collect(),
        (0..STOP_COUNT)
            .map(|_| Stop::new(random_location()))
            .collect(),
    )
}

fn create_cost_calculator(
    problem: &SimpleProblem,
) -> DeliveryCostCalculator<&CrowRouter, &SimpleProblem> {
    DeliveryCostCalculator::new(
        DistanceCostCalculator::new(&ROUTER, problem),
        problem.stops().len(),
        MISSED_DELIVERY_COST,
        DISTANCE_COST,
    )
}

fn dynamic_programming(bencher: &mut Bencher) {
    let problem = random_problem();
    let mut solver = DynamicProgrammingSolver::new(create_cost_calculator(&problem));

    bencher.iter(|| solver.solve(&problem));
}

fn branch_and_bound(bencher: &mut Bencher) {
    let problem = random_problem();
    let mut solver = BranchAndBoundSolver::new(create_cost_calculator(&problem));

    bencher.iter(|| solver.solve(&problem));
}

fn ruin_and_recreate(bencher: &mut Bencher) {
    let problem = random_problem();
    let mut solver = RuinAndRecreateSolver::new(
        create_cost_calculator(&problem),
        &ROUTER,
        NearestNeighborSolver::new(&ROUTER),
        ITERATION_COUNT,
    );

    bencher.iter(|| solver.solve(&problem));
}

fn benchmark(criterion: &mut Criterion) {
    criterion.bench_function("dynamic programming", dynamic_programming);
    criterion.bench_function("branch and bound", branch_and_bound);
    criterion.bench_function("ruin and recreate", ruin_and_recreate);
}

criterion_group!(benchmark_group, benchmark);
criterion_main!(benchmark_group);
