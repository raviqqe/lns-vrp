use itertools::Itertools;

pub fn permutations<T: Clone>(
    xs: impl IntoIterator<Item = T, IntoIter = impl Iterator<Item = T> + Clone>,
) -> impl Iterator<Item = [T; 2]> {
    let xs = xs.into_iter();
    let ys = xs.clone();

    xs.cartesian_product(ys).map(|(x, y)| [x, y])
}
