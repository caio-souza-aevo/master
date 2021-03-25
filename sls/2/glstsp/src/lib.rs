use crate::types::graph::Graph;
use crate::types::point::Point;

pub mod types;

pub fn load_data() -> Vec<Point> {
    let tsp = include_str!("../data/pcb3038.preprocessed.tsp");
    tsp
        .lines()
        .map(Point::from)
        .collect::<Vec<_>>()
}

pub fn load_problem() -> Graph {
    let tsp = load_data();
    Graph::new(&tsp)
}

pub fn main() {
    let tsp = load_problem();
    let solution = tsp.gls(666);

    // Optimal solution
    assert!(solution.cost >= 137694);
    println!("{:?}", solution);
}

#[cfg(test)]
mod tests_point {
    use crate::main;

    #[test]
    fn explore() {
        main();
    }
}
