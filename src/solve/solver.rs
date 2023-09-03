use crate::{Problem, Solution};

pub trait Solver {
    fn solve(&mut self, problem: &Problem) -> Solution;
}
