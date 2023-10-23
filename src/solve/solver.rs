use crate::{problem::BasicProblem, Solution};

pub trait Solver {
    fn solve(&mut self, problem: impl BasicProblem) -> Solution;
}
