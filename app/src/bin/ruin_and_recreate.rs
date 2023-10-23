use vrp::{
    bin_utility::{
        create_cost_calculator, create_router, measure_time, print_solution, random_problem,
    },
    solve::{NearestNeighborSolver, RuinAndRecreateSolver, Solver},
};

const MOVING_AVERAGE_DATA_POINT_COUNT: usize = 200;

fn main() {
    let router = create_router();
    let problem = random_problem(10, 100);
    let mut solver = RuinAndRecreateSolver::new(
        create_cost_calculator(&router, &problem),
        &router,
        NearestNeighborSolver::new(&router),
        MOVING_AVERAGE_DATA_POINT_COUNT,
    );

    let solution = measure_time(|| solver.solve(&problem));

    print_solution(&problem, &solution);
}
