use crate::{problem::BasicProblem, Solution};

pub trait Solver<P: BasicProblem> {
    fn solve(&mut self, problem: P) -> Solution;
}
