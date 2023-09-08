use std::{env::args, error::Error, fs::read_to_string};
use vrp::{SimpleProblem, Solution};

fn main() -> Result<(), Box<dyn Error>> {
    let args = args().collect::<Vec<_>>();
    let problem = SimpleProblem::from_json(serde_json::from_str(&read_to_string(&args[0])?)?)?;
    let solution = Solution::from_json(serde_json::from_str(&read_to_string(&args[1])?)?)?;

    println!("{}", solution.to_geojson(&problem));

    Ok(())
}
