#![feature(in_band_lifetimes)]

use crate::types::gls::GuidedLocalSearch;
use crate::types::point::Point;

pub mod types;

pub fn load_data() -> Vec<Point> {
    let tsp = include_str!("../data/pcb3038.preprocessed.tsp");
    tsp
        .lines()
        .map(Point::from)
        .collect::<Vec<_>>()
}

pub fn load_problem() -> GuidedLocalSearch {
    let tsp = load_data();
    GuidedLocalSearch::new(&tsp)
}

#[cfg(test)]
mod main {
    use crate::load_problem;

    #[test]
    fn write() {
        let tsp = load_problem();
        println!("{:.25}", tsp);
    }

    #[test]
    fn sequential() {
        let tsp = load_problem();
        let solution = tsp.sequential_route();

        // Optimal solution
        assert!(solution.cost() >= 137694);
        println!("{:?}", solution);
    }

    #[test]
    fn nn() {
        let tsp = load_problem();
        let solution = tsp.nearest_neighbor();

        // Optimal solution
        assert!(solution.cost() >= 137694);
        println!("{:?}", solution);
    }

    #[test]
    fn gls() {
        let tsp = load_problem();
        let solution = tsp.solve(666);

        // Optimal solution
        assert!(solution.cost() >= 137694);
        println!("{:?}", solution);
    }
}
