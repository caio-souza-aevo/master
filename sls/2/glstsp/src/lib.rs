#![feature(in_band_lifetimes)]

use crate::types::gls::GuidedLocalSearch;
use crate::types::point::Point;
use crate::types::matrix::SymmetricMatrix;

pub mod types;

pub fn load_matrix() -> SymmetricMatrix {
    let tsp = include_str!("../data/pcb3038.preprocessed.tsp");
    let tsp = tsp
        .lines()
        .map(Point::from)
        .collect::<Vec<_>>();
    SymmetricMatrix::from_euclidean_coords(&tsp)
}

pub fn load_problem() -> GuidedLocalSearch {
    let matrix = load_matrix();
    GuidedLocalSearch::new(matrix)
}

pub fn gls(steps: usize, expected: i32) {
    let tsp = load_problem();
    let solution = tsp.solve(666, steps);

    // Optimal solution
    assert_eq!(solution.cost, expected);
    println!("{:?}", solution);
}

#[cfg(test)]
mod main {
    use crate::gls;

    #[test]
    fn gls0() {
        gls(0, 152991);
    }

    #[test]
    fn gls1() {
        gls(1, 152979);
    }

    #[test]
    fn gls10() {
        gls(10, 152979);
    }

    #[test]
    fn gls25() {
        gls(25, 152777);
    }
}
