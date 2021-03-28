use std::ops::{Index, IndexMut};
use std::iter;

#[derive(Eq, PartialEq, Debug)]
pub struct Path(Vec<usize>);

#[derive(Eq, PartialEq, Debug)]
pub enum HamiltonianResult {
    Ok,
    VisitedTwice(usize),
}

impl Path
{
    pub fn new(path: Vec<usize>) -> Self {
        debug_assert!(path.len() > 1);
        Self(path)
    }

    pub fn from_size(size: usize) -> Self {
        Self::new(vec![0usize; size])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn sequential(size: usize) -> Self {
        let path: Vec<_> = (0..size).collect();
        Self::new(path)
    }

    /// Check if the path is complete and Hamiltonian
    pub fn check_hamiltonian(&self) -> HamiltonianResult {
        let mut visited = vec![false; self.0.len()];

        for vertex in self.0.iter().copied() {
            if visited[vertex] {
                return HamiltonianResult::VisitedTwice(vertex);
            }
            visited[vertex] = true;
        }

        HamiltonianResult::Ok
    }

    pub fn is_hamiltonian(&self) -> bool {
        self.check_hamiltonian() == HamiltonianResult::Ok
    }

    /// Twist the path from `i` to `j` both inclusive.
    pub fn twist(&mut self, i: usize, j: usize) {
        let mut i = i;
        let mut j = j;

        if i <= j {
            while i < j {
                self.0.swap(i, j);
                i += 1;
                j -= 1;
            }
        } else {
            let len = self.0.len();

            let middle = i + (len - (i - j + 1)) / 2;
            let middle = middle % len;

            loop {
                self.0.swap(i, j);
                if i == middle { break; }

                i = (i + 1) % len;
                j = (j + len - 1) % len;
            }
        }

        debug_assert!(self.is_hamiltonian());
    }

    /// Create an iterator of edges grouping each vertex with the next one.
    ///
    /// `Path::new(vec![2, 0, 1, 3]).edges()` should return an iterator equivalent to
    /// `[(2, 0), (0, 1), (1, 3), (3, 2)]`
    pub fn edges(&'_ self) -> impl Iterator<Item=(usize, usize)> + '_ {
        self.0.iter().copied()
            .zip(self.0.iter().copied()
                .skip(1)
                .chain(iter::once(self.0[0]))
            )
    }

    /// Create an iterator of edges interpolating all vertices with the next ones skipping `skip` amount.
    ///
    /// `Path::new(vec![2, 0, 1, 3]).interpolate_edges(skip: 0)` should return an iterator equivalent to
    /// `[(2, 0), (2, 1), (2, 3), (0, 1), (0, 3), (1, 3)]`
    ///
    /// `Path::new(vec![2, 0, 1, 3]).interpolate_edges(skip: 1)` should return an iterator equivalent to
    /// `[(2, 1), (2, 3), (1, 3)]`
    pub fn interpolate_edges(&'_ self, skip: usize) -> impl Iterator<Item=(usize, usize)> + '_ {
        self.0.iter().copied().enumerate()
            .flat_map(move |(i, v)| self.0.iter().copied()
                .skip(i + 1 + skip)
                .map(move |next_v| (v, next_v))
            )
    }
}

impl Index<usize> for Path {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.0.len());
        unsafe { self.0.get_unchecked(index) }
    }
}

impl IndexMut<usize> for Path {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.0.len());
        unsafe { self.0.get_unchecked_mut(index) }
    }
}

impl IntoIterator for Path {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::path::Path;

    fn create_path() -> Path {
        Path::new(vec![0, 1, 2, 3, 4, 5, 6, 7])
    }

    #[cfg(test)]
    mod hamiltonian {
        use crate::types::path::{Path, HamiltonianResult};

        #[test]
        fn all() {
            let path = Path(vec![0, 1, 2, 3, 4, 5, 6, 7]);
            assert_eq!(path.check_hamiltonian(), HamiltonianResult::Ok);
            assert!(path.is_hamiltonian());
        }

        #[test]
        fn first_is_repeated() {
            let path = Path(vec![1, 1, 2, 3, 4, 5, 6, 7]);
            assert_eq!(path.check_hamiltonian(), HamiltonianResult::VisitedTwice(1));
        }

        #[test]
        fn second_is_repeated() {
            let path = Path(vec![0, 1, 2, 3, 4, 2, 6, 7]);
            assert_eq!(path.check_hamiltonian(), HamiltonianResult::VisitedTwice(2));
        }

        #[test]
        fn internal_cycle() {
            let path = Path(vec![0, 1, 2, 3, 4, 5, 6, 0]);
            assert_eq!(path.check_hamiltonian(), HamiltonianResult::VisitedTwice(0));
        }
    }

    #[cfg(test)]
    mod inside_twist {
        use crate::types::path::tests::create_path;
        use crate::types::path::Path;

        #[test]
        fn twist_all() {
            let mut actual = create_path();
            actual.twist(0, 7);
            assert_eq!(actual, Path(vec![7, 6, 5, 4, 3, 2, 1, 0]));
        }

        #[test]
        fn twist_all_but_last() {
            let mut actual = create_path();
            actual.twist(0, 6);
            assert_eq!(actual, Path(vec![6, 5, 4, 3, 2, 1, 0, 7]));
        }

        #[test]
        fn twist_one() {
            let mut actual = create_path();
            actual.twist(2, 3);
            assert_eq!(actual, Path(vec![0, 1, 3, 2, 4, 5, 6, 7]));
        }

        #[test]
        fn twist_odd() {
            let mut actual = create_path();
            actual.twist(2, 5);
            assert_eq!(actual, Path(vec![0, 1, 5, 4, 3, 2, 6, 7]));
        }

        #[test]
        fn twist_even() {
            let mut actual = create_path();
            actual.twist(2, 4);
            assert_eq!(actual, Path(vec![0, 1, 4, 3, 2, 5, 6, 7]));
        }

        #[test]
        fn multiple_twists() {
            let mut actual = create_path();

            actual.twist(2, 4);
            assert_eq!(actual, Path(vec![0, 1, 4, 3, 2, 5, 6, 7]));

            actual.twist(3, 7);
            assert_eq!(actual, Path(vec![0, 1, 4, 7, 6, 5, 2, 3]));

            actual.twist(6, 7);
            assert_eq!(actual, Path(vec![0, 1, 4, 7, 6, 5, 3, 2]));
        }
    }

    #[cfg(test)]
    mod outside_twist {
        use crate::types::path::tests::create_path;
        use crate::types::path::Path;

        #[test]
        fn twist_all() {
            let mut actual = create_path();
            actual.twist(7, 0);
            assert_eq!(actual, Path(vec![7, 1, 2, 3, 4, 5, 6, 0]));
        }

        #[test]
        fn twist_all_but_last() {
            let mut actual = create_path();
            actual.twist(6, 0);
            assert_eq!(actual, Path(vec![6, 1, 2, 3, 4, 5, 0, 7]));
        }

        #[test]
        fn twist_one() {
            let mut actual = create_path();
            actual.twist(3, 2);
            assert_eq!(actual, Path(vec![5, 4, 3, 2, 1, 0, 7, 6]));
        }

        #[test]
        fn twist_odd() {
            let mut actual = create_path();
            actual.twist(5, 2);
            assert_eq!(actual, Path(vec![7, 6, 5, 3, 4, 2, 1, 0]));
        }

        #[test]
        fn twist_even() {
            let mut actual = create_path();
            actual.twist(4, 2);
            assert_eq!(actual, Path(vec![6, 5, 4, 3, 2, 1, 0, 7]));
        }

        #[test]
        fn multiple_twists() {
            let mut actual = create_path();

            actual.twist(4, 2);
            assert_eq!(actual, Path(vec![6, 5, 4, 3, 2, 1, 0, 7]));

            actual.twist(7, 3);
            assert_eq!(actual, Path(vec![4, 5, 6, 7, 2, 1, 0, 3]));

            actual.twist(7, 6);
            assert_eq!(actual, Path(vec![1, 2, 7, 6, 5, 4, 3, 0]));
        }
    }

    #[cfg(test)]
    mod edges {
        use crate::types::path::Path;

        #[test]
        // This is a special case when the route has only two vertices.
        // Not worth handling because this is never the case in a real world scenario and
        // would only impact the performance for extreme small instances.
        fn edges_single() {
            let path = Path(vec![0, 1]);

            let actual = path.edges();
            let expected = vec![(0, 1), (1, 0)];

            itertools::assert_equal(actual, expected);
        }

        #[test]
        fn edges() {
            let path = Path(vec![2, 0, 1, 3]);

            let actual = path.edges();
            let expected = vec![(2, 0), (0, 1), (1, 3), (3, 2)];

            itertools::assert_equal(actual, expected);
        }
    }

    #[cfg(test)]
    mod interpolate_edges {
        use crate::types::path::Path;

        #[test]
        fn edges_single() {
            let path = Path(vec![0, 1]);

            let actual = path.interpolate_edges(0);
            let expected = vec![(0, 1)];

            itertools::assert_equal(actual, expected);
        }

        #[test]
        fn interpolate_no_skip() {
            let path = Path(vec![2, 0, 1, 3]);

            let actual = path.interpolate_edges(0);
            let expected = vec![(2, 0), (2, 1), (2, 3), (0, 1), (0, 3), (1, 3)];

            itertools::assert_equal(actual, expected);
        }

        #[test]
        fn interpolate_skip_1() {
            let path = Path(vec![2, 0, 1, 3]);

            let actual = path.interpolate_edges(1);
            let expected = vec![(2, 1), (2, 3), (0, 3)];

            itertools::assert_equal(actual, expected);
        }

        #[test]
        fn interpolate_skip_2() {
            let path = Path(vec![2, 0, 1, 3]);

            let actual = path.interpolate_edges(2);
            let expected = vec![(2, 3)];

            itertools::assert_equal(actual, expected);
        }

        #[test]
        fn interpolate_skip_all() {
            let path = Path(vec![2, 0, 1, 3]);

            let actual = path.interpolate_edges(3);
            let expected = vec![];

            itertools::assert_equal(actual, expected);
        }
    }
}
