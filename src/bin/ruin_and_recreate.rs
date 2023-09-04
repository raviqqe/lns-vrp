use vrp::{
    bin_utility::{create_cost_calculator, random_problem},
    solve::{RuinAndRecreateSolver, Solver},
};

const ITERATION_COUNT: usize = 100;

fn main() {
    let problem = random_problem();
    let mut solver = RuinAndRecreateSolver::new(create_cost_calculator(&problem), ITERATION_COUNT);

    dbg!(solver
        .solve(&problem)
        .routes()
        .iter()
        .map(|indexes| indexes
            .iter()
            .map(|index| problem.stops()[*index].location().as_point().x())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>());
}
