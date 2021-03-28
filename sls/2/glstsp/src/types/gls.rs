use crate::types::matrix::SymmetricMatrix;
use crate::types::route::Route;
use crate::types::path::Path;
use rand_mt::Mt64;
use rand::SeedableRng;
use rand::seq::SliceRandom;

#[derive(Eq, PartialEq)]
pub struct GuidedLocalSearch {
    distances: SymmetricMatrix,
}

impl GuidedLocalSearch {
    pub fn new(distances: SymmetricMatrix) -> Self {
        Self { distances }
    }

    fn cost(&self, path: &Path) -> i32 {
        self.distances.sum(path.edges())
    }

    pub fn sequential(&self) -> Route {
        let path = Path::sequential(self.distances.size());
        let cost = self.cost(&path);
        Route::new(path, cost)
    }

    pub fn nearest_neighbor(&self) -> Route {
        let size = self.distances.size();

        let mut res = Path::from_size(size);
        let mut remainders: Vec<_> = (1..size).collect();

        for i in 0..size - 1 {
            let (remainder, neighbor) = remainders.iter().copied()
                .enumerate()
                .min_by(|&(_, n_a), &(_, n_b)|
                    self.distances[(i, n_a)].cmp(&self.distances[(i, n_b)])
                )
                .unwrap();

            remainders.remove(remainder);
            res[i + 1] = neighbor;
        }

        let cost = self.cost(&res);
        let res = Route::new(res, cost);

        debug_assert!(res.path.is_hamiltonian());

        res
    }

    fn local_search_step(
        &self,
        candidate: &mut Route,
        neighborhood: &Path,
        penalty_factor: i32,
        penalties: &mut SymmetricMatrix,
    ) -> i32 {
        let cost_change = |va: (usize, usize), vb: (usize, usize)| {
            self.distances[va] + self.distances[vb]
                + penalty_factor * (penalties[va] + penalties[vb])
        };

        let candidate_path = &candidate.path;

        for (i, j) in neighborhood.interpolate_edges(1) {

            // Find vertexes to twist
            let i_next = (i + 1) % candidate_path.len();
            let i_vertex = candidate_path[i];
            let i_vertex_next = candidate_path[i_next];

            let j_next = (j + 1) % candidate_path.len();
            let j_vertex = candidate_path[j];
            let j_vertex_next = candidate_path[j_next];

            // Calculate the new cost: {i, i+1}, {j, j+1} -> {i, j}, {i+1, j+1}
            let cost_decreased = cost_change((i_vertex, i_vertex_next), (j_vertex, j_vertex_next));
            let cost_increased = cost_change((i_vertex, j_vertex), (i_vertex_next, j_vertex_next));
            let cost_change = cost_increased - cost_decreased;

            // If the cost is decreased, apply the twist and finish the step
            if cost_change < 0 {
                candidate.path.twist(i_next, j);
                candidate.cost += cost_change;
                return cost_change;
            }
        }

        0
    }

    pub fn local_search(
        &self,
        candidate: &mut Route,
        neighborhood: &Path,
        penalty_factor: i32,
        penalties: &mut SymmetricMatrix)
    {
        // Recalculate the cost considering the penalties
        candidate.cost =
            self.cost(&candidate.path)
                + penalty_factor * penalties.sum(candidate.path.edges());

        loop {
            let change = self.local_search_step(candidate, &neighborhood, penalty_factor, penalties);
            if change == 0 { break; }
        }
    }

    pub fn solve(&self, seed: u64) -> Route {
        let size = self.distances.size();

        // RNG
        let mut rng: Mt64 = SeedableRng::seed_from_u64(seed);

        // Neighborhood search
        let mut neighborhood: Vec<_> = (0..size).collect();
        neighborhood.shuffle(&mut rng);
        let neighborhood = Path::new(neighborhood);

        // Penalties
        let mut penalties = SymmetricMatrix::from_size(size);

        let mut route = self.nearest_neighbor();
        self.local_search(&mut route, &neighborhood, 0, &mut penalties);
        route
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
