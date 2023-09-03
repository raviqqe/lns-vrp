use crate::{Problem, Solution};

pub trait Solver {
    fn solve(&self, problem: &Problem) -> Solution;
}
