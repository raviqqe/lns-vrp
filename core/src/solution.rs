pub trait BasicSolution {
    fn routes(&self) -> impl Iterator<Item = impl Iterator<Item = usize>>;
}
