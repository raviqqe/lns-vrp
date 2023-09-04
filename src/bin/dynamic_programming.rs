use vrp::{
    bin_utility::{create_cost_calculator, measure_time, print_solution, random_problem},
    solve::{DynamicProgrammingSolver, Solver},
};

fn main() {
    let problem = random_problem();
    let mut solver = DynamicProgrammingSolver::new(create_cost_calculator(&problem));

    let solution = measure_time(|| solver.solve(&problem));

    print_solution(&problem, &solution);
}
