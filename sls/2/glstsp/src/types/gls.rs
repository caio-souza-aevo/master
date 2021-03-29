use crate::types::matrix::SymmetricMatrix;
use crate::types::route::Route;
use crate::types::path::Path;
use rand_mt::Mt64;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rayon::prelude::*;

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
        Route::new(cost, path)
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
        let res = Route::new(cost, res);

        debug_assert!(res.path.is_hamiltonian());

        res
    }

    pub fn local_search(
        &self,
        candidate: &mut Path,
        neighborhood: &Path,
        penalty_factor: i32,
        penalties: &mut SymmetricMatrix)
    {
        let cost_change = |va: (usize, usize), vb: (usize, usize)| {
            self.distances[va] + self.distances[vb]
                + penalty_factor * (penalties[va] + penalties[vb])
        };

        loop {
            let twist = neighborhood.0
                .par_iter()
                .enumerate()
                .map(|(skip, &i)| {
                    // Find vertexes to twist
                    let i_next = (i + 1) % candidate.len();
                    let i_vertex = candidate[i];
                    let i_vertex_next = candidate[i_next];

                    for j in neighborhood.0.iter().copied().skip(skip + 2) {
                        let j_next = (j + 1) % candidate.len();
                        let j_vertex = candidate[j];
                        let j_vertex_next = candidate[j_next];

                        // Calculate the new cost: {i, i+1}, {j, j+1} -> {i, j}, {i+1, j+1}
                        let cost_decreased = cost_change((i_vertex, i_vertex_next), (j_vertex, j_vertex_next));
                        let cost_increased = cost_change((i_vertex, j_vertex), (i_vertex_next, j_vertex_next));
                        let cost_change = cost_increased - cost_decreased;

                        // If the cost is decreased, apply the twist and finish the step
                        if cost_change < 0 {
                            return Some((i_next, j));
                        }
                    }

                    None
                })
                .find_first(|&r| r != None);

            match twist {
                None | Some(None) => { return; } // No improvement found. Already in local minimum.
                Some(Some((e0, e1))) => { candidate.twist(e0, e1) } // Apply the twist
            }
        }
    }

    pub fn solve(&self, seed: u64, steps: usize) -> Route {
        let size = self.distances.size();

        // RNG
        let mut rng: Mt64 = SeedableRng::seed_from_u64(seed);

        // Neighborhood search
        let mut neighborhood: Vec<_> = (0..size).collect();
        neighborhood.shuffle(&mut rng);
        let neighborhood = &Path::new(neighborhood);

        // Candidate
        let mut route = self.nearest_neighbor();

        // First iteration
        let mut penalties = SymmetricMatrix::from_size(size);
        self.local_search(&mut route.path, neighborhood, 0, &mut penalties);
        route.cost = self.cost(&route.path);

        let penalty_factor = (0.3 * (route.cost as f64 / size as f64)) as i32;

        for _ in 0..steps {
            let calc_utility = |penalties: &SymmetricMatrix, e: (usize, usize)| -> i32 {
                (self.distances[e] as f64 / (1.0 + penalties[e] as f64)) as i32
            };

            // Find the maximum utility
            let max_utility = route.path.edges()
                .par_bridge()
                .map(|e| calc_utility(&penalties, e))
                .max()
                .unwrap();

            // Penalize features with maximum utility
            route.path.edges()
                .par_bridge()
                .filter(|&e| calc_utility(&penalties, e) == max_utility)
                .collect::<Vec<_>>()
                .iter().for_each(|&(e0, e1)| penalties.inc(e0, e1, 1));

            self.local_search(&mut route.path, neighborhood, penalty_factor, &mut penalties);
        }

        // Run a last local search pass without penalties to reach the local minimum
        self.local_search(&mut route.path, neighborhood, 0, &mut penalties);
        route.cost = self.cost(&route.path);
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

            let expected = Route::new(18, Path::new(vec![0, 1, 2, 3]));

            assert_eq!(actual, expected);
        }
    }
}
