use std::ops::Index;
use std::iter;

#[derive(Eq, PartialEq, Debug)]
pub struct Route {
    path: Vec<usize>,
    cost: i32,
}

#[derive(Eq, PartialEq, Debug)]
pub enum HamiltonianResult {
    Ok,
    VisitedTwice(usize),
}

impl Route
{
    pub fn new(path: Vec<usize>, cost: i32) -> Route {
        debug_assert!(path.len() > 1);
        Route { path, cost }
    }

    pub fn cost(&self) -> i32 {
        self.cost
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn len(&self) -> usize {
        self.path.len()
    }

    /// Check if the route is complete and Hamiltonian
    pub fn check_hamiltonian(&self) -> HamiltonianResult {
        let mut visited = vec![false; self.path.len()];

        for vertex in self.path.iter().copied() {
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

    /// Twist the route from `i` to `j` both inclusive and apply the `cost_change` of the twist.
    pub fn twist(&mut self, i: usize, j: usize, cost_change: i32) {
        self.cost += cost_change;

        let mut i = i;
        let mut j = j;

        if i <= j {
            while i < j {
                self.path.swap(i, j);
                i += 1;
                j -= 1;
            }
        } else {
            let len = self.len();

            let middle = i + (len - (i - j + 1)) / 2;
            let middle = middle % len;

            loop {
                self.path.swap(i, j);
                if i == middle { break; }

                i = (i + 1) % len;
                j = (j + len - 1) % len;
            }
        }

        debug_assert!(self.is_hamiltonian());
    }

    /// Create an iterator of edges grouping each vertex with the next one.
    ///
    /// See [`edges`] for more information.
    pub fn edge_from_vertices(vertices: &'_ [usize]) -> impl Iterator<Item=(usize, usize)> + '_ {
        vertices.iter().copied()
            .zip(vertices.iter().copied()
                .skip(1)
                .chain(iter::once(vertices[0]))
            )
    }

    /// Create an iterator of edges grouping each vertex with the next one.
    ///
    /// `Route(vec![2, 0, 1, 3])` should return an iterator equivalent to
    /// `[(2, 0), (0, 1), (1, 3), (3, 2)]`
    pub fn edges(&'_ self) -> impl Iterator<Item=(usize, usize)> + '_ {
        Self::edge_from_vertices(&self.path)
    }
}

impl Index<usize> for Route {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.len());
        unsafe { self.path.get_unchecked(index) }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::route::Route;

    fn create_route() -> Route {
        Route {
            path: vec![0, 1, 2, 3, 4, 5, 6, 7],
            cost: 5,
        }
    }

    #[cfg(test)]
    mod hamiltonian {
        use crate::types::route::{Route, HamiltonianResult};

        #[test]
        fn all() {
            let route = Route {
                path: vec![0, 1, 2, 3, 4, 5, 6, 7],
                cost: 5,
            };
            assert_eq!(route.check_hamiltonian(), HamiltonianResult::Ok);
            assert!(route.is_hamiltonian());
        }

        #[test]
        fn first_is_repeated() {
            let route = Route {
                path: vec![1, 1, 2, 3, 4, 5, 6, 7],
                cost: 5,
            };
            assert_eq!(route.check_hamiltonian(), HamiltonianResult::VisitedTwice(1));
        }

        #[test]
        fn second_is_repeated() {
            let route = Route {
                path: vec![0, 1, 2, 3, 4, 2, 6, 7],
                cost: 5,
            };
            assert_eq!(route.check_hamiltonian(), HamiltonianResult::VisitedTwice(2));
        }

        #[test]
        fn internal_cycle() {
            let route = Route {
                path: vec![0, 1, 2, 3, 4, 5, 6, 0],
                cost: 5,
            };
            assert_eq!(route.check_hamiltonian(), HamiltonianResult::VisitedTwice(0));
        }
    }

    #[cfg(test)]
    mod inside_twist {
        use crate::types::route::tests::create_route;
        use crate::types::route::Route;

        #[test]
        fn twist_all() {
            let mut actual = create_route();
            actual.twist(0, 7, 10);

            let expected = Route {
                path: vec![7, 6, 5, 4, 3, 2, 1, 0],
                cost: 15,
            };

            assert_eq!(actual, expected);
        }

        #[test]
        fn twist_all_but_last() {
            let mut actual = create_route();
            actual.twist(0, 6, 10);

            let expected = Route {
                path: vec![6, 5, 4, 3, 2, 1, 0, 7],
                cost: 15,
            };

            assert_eq!(actual, expected);
        }

        #[test]
        fn twist_one() {
            let mut actual = create_route();
            actual.twist(2, 3, 10);

            let expected = Route {
                path: vec![0, 1, 3, 2, 4, 5, 6, 7],
                cost: 15,
            };

            assert_eq!(actual, expected);
        }

        #[test]
        fn twist_odd() {
            let mut actual = create_route();
            actual.twist(2, 5, 10);

            let expected = Route {
                path: vec![0, 1, 5, 4, 3, 2, 6, 7],
                cost: 15,
            };

            assert_eq!(actual, expected);
        }

        #[test]
        fn twist_even() {
            let mut actual = create_route();
            actual.twist(2, 4, 10);

            let expected = Route {
                path: vec![0, 1, 4, 3, 2, 5, 6, 7],
                cost: 15,
            };

            assert_eq!(actual, expected);
        }

        #[test]
        fn multiple_twists() {
            let mut actual = create_route();
            actual.twist(2, 4, 10);
            let expected = Route {
                path: vec![0, 1, 4, 3, 2, 5, 6, 7],
                cost: 15,
            };
            assert_eq!(actual, expected);


            actual.twist(3, 7, -1);
            let expected = Route {
                path: vec![0, 1, 4, 7, 6, 5, 2, 3],
                cost: 14,
            };
            assert_eq!(actual, expected);

            actual.twist(6, 7, -1);
            let expected = Route {
                path: vec![0, 1, 4, 7, 6, 5, 3, 2],
                cost: 13,
            };
            assert_eq!(actual, expected);
        }
    }

    #[cfg(test)]
    mod outside_twist {
        use crate::types::route::tests::create_route;
        use crate::types::route::Route;

        #[test]
        fn twist_all() {
            let mut actual = create_route();
            actual.twist(7, 0, 10);

            let expected = Route {
                path: vec![7, 1, 2, 3, 4, 5, 6, 0],
                cost: 15,
            };

            assert_eq!(actual, expected);
        }

        #[test]
        fn twist_all_but_last() {
            let mut actual = create_route();
            actual.twist(6, 0, 10);

            let expected = Route {
                path: vec![6, 1, 2, 3, 4, 5, 0, 7],
                cost: 15,
            };

            assert_eq!(actual, expected);
        }

        #[test]
        fn twist_one() {
            let mut actual = create_route();
            actual.twist(3, 2, 10);

            let expected = Route {
                path: vec![5, 4, 3, 2, 1, 0, 7, 6],
                cost: 15,
            };

            assert_eq!(actual, expected);
        }

        #[test]
        fn twist_odd() {
            let mut actual = create_route();
            actual.twist(5, 2, 10);

            let expected = Route {
                path: vec![7, 6, 5, 3, 4, 2, 1, 0],
                cost: 15,
            };

            assert_eq!(actual, expected);
        }

        #[test]
        fn twist_even() {
            let mut actual = create_route();
            actual.twist(4, 2, 10);

            let expected = Route {
                path: vec![6, 5, 4, 3, 2, 1, 0, 7],
                cost: 15,
            };

            assert_eq!(actual, expected);
        }

        #[test]
        fn multiple_twists() {
            let mut actual = create_route();

            actual.twist(4, 2, 10);
            let expected = Route {
                path: vec![6, 5, 4, 3, 2, 1, 0, 7],
                cost: 15,
            };
            assert_eq!(actual, expected);

            actual.twist(7, 3, -1);
            let expected = Route {
                path: vec![4, 5, 6, 7, 2, 1, 0, 3],
                cost: 14,
            };
            assert_eq!(actual, expected);

            actual.twist(7, 6, -1);
            let expected = Route {
                path: vec![1, 2, 7, 6, 5, 4, 3, 0],
                cost: 13,
            };
            assert_eq!(actual, expected);
        }
    }

    #[cfg(test)]
    mod edges {
        use crate::types::route::Route;

        #[test]
        // This is a special case when the route has only two vertices.
        // Not worth handling because this is never the case in a real world scenario and
        // would only impact the performance for extreme small instances.
        fn edges_single() {
            let route = Route {
                cost: 0,
                path: vec![0, 1],
            };

            let actual = route.edges();
            let expected = vec![(0, 1), (1, 0)];

            itertools::assert_equal(actual, expected);
        }

        #[test]
        fn edges() {
            let route = Route {
                cost: 0,
                path: vec![2, 0, 1, 3],
            };

            let actual = route.edges();
            let expected = vec![(2, 0), (0, 1), (1, 3), (3, 2)];

            itertools::assert_equal(actual, expected);
        }
    }
}
