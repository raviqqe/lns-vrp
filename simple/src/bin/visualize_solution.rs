use lns_vrp_simple::{Problem, Solution};
use std::{env::args, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let args = args().collect::<Vec<_>>();
    let problem = Problem::from_json(serde_json::from_str(&read_to_string(&args[1])?)?)?;
    let solution = Solution::from_json(serde_json::from_str(&read_to_string(&args[2])?)?)?;

    println!("{}", solution.to_geojson(&problem));

    Ok(())
}
