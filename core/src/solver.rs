use crate::{problem::BasicProblem, BasicSolution, BasicStop, BasicVehicle};

pub trait BasicSolver<V: BasicVehicle, S: BasicStop, P: BasicProblem<V, S>, L: BasicSolution> {
    fn solve(&mut self, problem: &P) -> L;
}
