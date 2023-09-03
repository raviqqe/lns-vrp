use crate::{SimpleProblem, Solution};

pub trait Solver {
    fn solve(&mut self, problem: &SimpleProblem) -> Solution;
}
