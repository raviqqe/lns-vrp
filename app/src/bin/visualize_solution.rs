use core::BasicSolution;
use lns_vrp_app::SimpleProblem;
use std::{env::args, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let args = args().collect::<Vec<_>>();
    let problem = SimpleProblem::from_json(serde_json::from_str(&read_to_string(&args[1])?)?)?;
    let solution = BasicSolution::from_json(serde_json::from_str(&read_to_string(&args[2])?)?)?;

    println!("{}", solution.to_geojson(&problem));

    Ok(())
}
