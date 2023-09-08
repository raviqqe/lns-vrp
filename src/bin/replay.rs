use std::{env::args, error::Error, fs::read_to_string};
use vrp::{
    bin_utility::{
        create_cost_calculator, create_router, measure_time, print_solution, random_problem,
    },
    solve::{NearestNeighborSolver, RuinAndRecreateSolver, Solver},
    SimpleProblem,
};

const MOVING_AVERAGE_DATA_POINT_COUNT: usize = 200;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args().collect::<Vec<_>>();
    let problem = SimpleProblem::from_json(serde_json::from_str(&read_to_string(&args[1])?)?)?;

    let router = create_router();
    let mut solver = RuinAndRecreateSolver::new(
        create_cost_calculator(&router, &problem),
        &router,
        NearestNeighborSolver::new(&router),
        MOVING_AVERAGE_DATA_POINT_COUNT,
    );

    let solution = measure_time(|| solver.solve(&problem));

    print_solution(&problem, &solution);

    Ok(())
}
