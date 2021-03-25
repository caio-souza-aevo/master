use std::ops::Index;

#[derive(Eq, PartialEq, Debug)]
pub struct Route {
    pub path: Vec<usize>,
    pub cost: i32,
}

impl Route
{
    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn twist(&mut self, i: usize, j: usize, cost_change: i32) {
        let mut i = i;
        let mut j = j;

        if i <= j {
            while i < j {
                self.path.swap(i, j);
                i += 1;
                j -= 1;
            }
        } else {
            let middle = i + (self.len() - (i - j + 1)) / 2;
            let middle = middle % self.len();

            loop {
                self.path.swap(i, j);
                if i == middle { break; }

                i = (i + 1) % self.len();
                j = (j + self.len() - 1) % self.len();
            }
        }

        self.cost += cost_change;
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
}
