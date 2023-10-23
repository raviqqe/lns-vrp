use core::{Location, Solver};
use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use lns_vrp_app::{
    cost::{DeliveryCostCalculator, DistanceCostCalculator},
    route::{CachedRouter, CrowRouter, Router},
    solve::{
        BranchAndBoundSolver, DynamicProgrammingSolver, NearestNeighborSolver,
        RuinAndRecreateSolver,
    },
    SimpleProblem, Stop, Vehicle,
};
use rand::random;

const STOP_COUNT: usize = 8;
const VEHICLE_COUNT: usize = 2;
const MOVING_AVERAGE_DATA_POINT_COUNT: usize = 100;

const DISTANCE_COST: f64 = 1.0;
const MISSED_DELIVERY_COST: f64 = 1e9;
const QUADRATIC_DISTANCE_COST: f64 = 1e-3;

fn create_router() -> CachedRouter<CrowRouter> {
    CachedRouter::new(CrowRouter::new())
}

fn random_longitude() -> f64 {
    0.1 * random::<f64>()
}

fn random_location() -> Location {
    Location::new(random_longitude(), 0.0)
}

fn random_problem() -> SimpleProblem {
    SimpleProblem::new(
        (0..VEHICLE_COUNT)
            .map(|_| Vehicle::new(STOP_COUNT, STOP_COUNT))
            .collect(),
        (0..STOP_COUNT).map(Stop::new).collect(),
        (0..STOP_COUNT + 1).map(|_| random_location()).collect(),
    )
}

fn create_cost_calculator(
    router: impl Router,
    problem: &SimpleProblem,
) -> DeliveryCostCalculator<impl Router, &SimpleProblem> {
    DeliveryCostCalculator::new(
        DistanceCostCalculator::new(router, problem),
        problem.stops().len(),
        MISSED_DELIVERY_COST,
        DISTANCE_COST,
        QUADRATIC_DISTANCE_COST,
    )
}

fn dynamic_programming(bencher: &mut Bencher) {
    let router = create_router();
    let problem = random_problem();
    let mut solver = DynamicProgrammingSolver::new(create_cost_calculator(&router, &problem));

    bencher.iter(|| solver.solve(&problem));
}

fn branch_and_bound(bencher: &mut Bencher) {
    let router = create_router();
    let problem = random_problem();
    let mut solver = BranchAndBoundSolver::new(create_cost_calculator(&router, &problem));

    bencher.iter(|| solver.solve(&problem));
}

fn ruin_and_recreate(bencher: &mut Bencher) {
    let router = create_router();
    let problem = random_problem();
    let mut solver = RuinAndRecreateSolver::new(
        create_cost_calculator(&router, &problem),
        &router,
        NearestNeighborSolver::new(&router),
        MOVING_AVERAGE_DATA_POINT_COUNT,
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
