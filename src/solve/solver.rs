use crate::Problem;

pub trait Solver {
    // TODO Reason no solution.
    fn solve(&self, problem: &Problem) -> Option<Problem>;
}
