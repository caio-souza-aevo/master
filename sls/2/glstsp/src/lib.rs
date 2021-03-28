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

#[cfg(test)]
mod main {
    use crate::{load_matrix, load_problem};

    #[test]
    fn write() {
        let tsp = load_matrix();
        println!("{:.25}", tsp);
    }

    #[test]
    fn sequential() {
        let tsp = load_problem();
        let solution = tsp.sequential();

        // Optimal solution
        assert!(solution.cost() >= 137694);
        println!("{:?}", solution);
    }
}
