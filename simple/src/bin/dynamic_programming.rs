use core::BasicSolver;
use lns_vrp_simple::{
    bin_utility::{
        create_cost_calculator, create_router, measure_time, print_solution, random_problem,
    },
    solve::DynamicProgrammingSolver,
};

fn main() {
    let router = create_router();
    let problem = random_problem(3, 8);
    let mut solver = DynamicProgrammingSolver::new(create_cost_calculator(&router, &problem));

    let solution = measure_time(|| solver.solve(&problem));

    print_solution(&problem, &solution);
}
