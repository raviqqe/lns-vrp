use core::Solver;
use lns_vrp_app::{
    bin_utility::{create_cost_calculator, create_router, measure_time, print_solution},
    solve::{NearestNeighborSolver, RuinAndRecreateSolver},
    SimpleProblem,
};
use std::{env::args, error::Error, fs::read_to_string};

const MOVING_AVERAGE_DATA_POINT_COUNT: usize = 1000;

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
