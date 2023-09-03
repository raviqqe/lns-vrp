use crate::{problem::BaseProblem, Solution};

pub trait Solver {
    fn solve(&mut self, problem: impl BaseProblem) -> Solution;
}
