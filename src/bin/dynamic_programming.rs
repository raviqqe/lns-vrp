use vrp::{
    bin_utility::{create_cost_calculator, random_problem},
    solve::{DynamicProgrammingSolver, Solver},
};

fn main() {
    let problem = random_problem();

    let mut solver = DynamicProgrammingSolver::new(create_cost_calculator(&problem));

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
