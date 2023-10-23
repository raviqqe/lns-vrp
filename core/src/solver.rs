use crate::{problem::BasicProblem, Solution};

pub trait BasicSolver<P: BasicProblem> {
    fn solve(&mut self, problem: P) -> Solution;
}
