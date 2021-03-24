use crate::types::graph::Graph;
use crate::types::point::Point;

pub mod types;

pub fn load_problem() -> Graph {
    let tsp = include_str!("../data/pcb3038.preprocessed.tsp");
    let tsp:Vec<_> = tsp
        .lines()
        .map(Point::from)
        .collect();

    Graph::new(&tsp)
}

#[cfg(test)]
mod tests_point {
    use crate::load_problem;

    #[test]
    fn sanity() {
        let tsp = load_problem();
        for seed in 0u64..100
        {
            // Optimal solution
            assert!(tsp.gls(seed).cost >= 137694);
        }
    }
}
