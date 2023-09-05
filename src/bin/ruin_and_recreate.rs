use vrp::{
    bin_utility::{create_cost_calculator, measure_time, print_solution, random_problem, ROUTER},
    solve::{NearestNeighborSolver, RuinAndRecreateSolver, Solver},
};

const ITERATION_COUNT: usize = 1000;

fn main() {
    let problem = random_problem(10, 100);
    let mut solver = RuinAndRecreateSolver::new(
        create_cost_calculator(&problem),
        &ROUTER,
        NearestNeighborSolver::new(&ROUTER),
        ITERATION_COUNT,
    );

    let solution = measure_time(|| solver.solve(&problem));

    print_solution(&problem, &solution);
}
