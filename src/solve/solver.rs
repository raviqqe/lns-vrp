pub trait Solver {
    fn solve(&self) -> Option<Problem>;
}
