pub trait Solution {
    fn routes(&self) -> impl Iterator<Item = impl Iterator<Item = usize>>;
}
