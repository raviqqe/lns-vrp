use itertools::Itertools;

pub fn permutations<T: Clone>(
    xs: impl IntoIterator<Item = T, IntoIter = impl Iterator<Item = T> + Clone>,
) -> impl Iterator<Item = [T; 2]> {
    let xs = xs.into_iter();
    let ys = xs.clone();

    xs.cartesian_product(ys).map(|(x, y)| [x, y])
}

#[cfg(test)]
mod tests {
    use super::*;

    mod permutations {
        use super::*;

        #[test]
        fn two_elements() {
            assert_eq!(
                permutations(0..2).collect::<Vec<_>>(),
                vec![[0, 0], [0, 1], [1, 0], [1, 1]]
            );
        }

        #[test]
        fn three_elements() {
            assert_eq!(
                permutations(0..3).collect::<Vec<_>>(),
                vec![
                    [0, 0],
                    [0, 1],
                    [0, 2],
                    [1, 0],
                    [1, 1],
                    [1, 2],
                    [2, 0],
                    [2, 1],
                    [2, 2]
                ]
            );
        }
    }
}
