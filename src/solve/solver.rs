use crate::Problem;

pub trait Solver {
    fn solve(&self, problem: &Problem) -> Solution;
}
