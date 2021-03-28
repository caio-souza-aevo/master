use crate::types::matrix::SymmetricMatrix;
use crate::types::route::Route;
use crate::types::path::Path;

#[derive(Eq, PartialEq)]
pub struct GuidedLocalSearch {
    distances: SymmetricMatrix,
}

impl GuidedLocalSearch {
    pub fn new(distances: SymmetricMatrix) -> Self {
        Self { distances }
    }

    pub fn sequential(&self) -> Route {
        let path = Path::sequential(self.distances.size());
        let cost = self.distances.sum(path.edges());
        Route::new(path, cost)
    }

    pub fn solve(&self, _seed: u64) -> Route {
        self.sequential()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod sequential {
        use crate::types::matrix::SymmetricMatrix;
        use crate::types::gls::GuidedLocalSearch;
        use crate::types::route::Route;
        use crate::types::path::Path;

        #[test]
        fn test() {
            let mut matrix = SymmetricMatrix::from_size(4);
            matrix.set(0, 1, 2);
            matrix.set(0, 2, 7);
            matrix.set(0, 3, 3);
            matrix.set(1, 2, 4);
            matrix.set(1, 3, 1);
            matrix.set(2, 3, 9);

            let gls = GuidedLocalSearch::new(matrix);
            let actual = gls.sequential();

            let expected = Route::new(
                Path::new(vec![0, 1, 2, 3]),
                18,
            );

            assert_eq!(actual, expected);
        }
    }
}
