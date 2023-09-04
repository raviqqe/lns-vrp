use vrp::{
    bin_utility::{create_cost_calculator, measure_time, print_solution, random_problem},
    solve::{RuinAndRecreateSolver, Solver},
};

const ITERATION_COUNT: usize = 100;

fn main() {
    let problem = random_problem();
    let mut solver = RuinAndRecreateSolver::new(create_cost_calculator(&problem), ITERATION_COUNT);

    let solution = measure_time(|| solver.solve(&problem));

    print_solution(&problem, &solution);
}
