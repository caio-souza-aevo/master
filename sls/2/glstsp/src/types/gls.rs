use std::ops::{Index, IndexMut};
use crate::types::point::Point;
use crate::types::route::Route;
use rand_mt::Mt64;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use std::fmt::{Formatter, Display};
use std::fmt;

#[derive(Eq, PartialEq)]
pub struct GuidedLocalSearch {
    size: usize,
    data: Vec<i32>,
    penalty: Vec<i32>,
}

impl GuidedLocalSearch {
    fn from_data(size: usize, data: Vec<i32>) -> GuidedLocalSearch {
        Self {
            size,
            data,
            penalty: vec![0i32; size * size],
        }
    }

    fn from_size(size: usize) -> GuidedLocalSearch {
        let data = vec![0i32; size * size];
        Self::from_data(size, data)
    }

    pub fn new(points: &[Point]) -> Self {
        let size = points.len();
        assert!(size > 0);

        let mut res = Self::from_size(size);

        for (i, point) in points.iter().copied().enumerate() {
            for (j, neighbor) in points.iter().copied().enumerate().skip(i + 1) {
                let dist = point.dist(neighbor);
                res[(i, j)] = dist;
                res[(j, i)] = dist;
            }
        }

        res
    }

    #[inline]
    fn get_index(&self, index: (usize, usize)) -> usize {
        let (x, y) = index;
        debug_assert!(x < self.size);
        debug_assert!(y < self.size);
        x * self.size + y
    }

    #[inline]
    fn penalty(&self, index: (usize, usize)) -> i32 {
        self.penalty[self.get_index(index)]
    }

    fn sum_edges(&self, edges: &[usize]) -> i32 {
        assert_eq!(edges.len(), self.size);

        let mut dist = 0;

        let indexes = edges.iter().zip(edges.iter().skip(1));
        for (&curr, &next) in indexes {
            dist += self[(curr, next)]
        }

        dist += self[(edges[edges.len() - 1], edges[0])];
        dist
    }

    pub fn sequential_route(&self) -> Route {
        let path: Vec<_> = (0..self.size).collect();
        let cost = self.sum_edges(&path);
        Route::new(path, cost)
    }

    pub fn nearest_neighbor(&self) -> Route {
        let mut res = vec![0usize; self.size];
        let mut remainders: Vec<_> = (1..self.size).collect();

        for i in 0..self.size - 1 {
            let (remainder, neighbor) = remainders
                .iter()
                .copied()
                .enumerate()
                .min_by(|&(_, n_a), &(_, n_b)|
                    self[(i, n_a)].cmp(&self[(i, n_b)])
                )
                .unwrap();

            remainders.remove(remainder);
            res[i + 1] = neighbor;
        }

        let cost = self.sum_edges(&res);
        let res = Route::new(res, cost);

        debug_assert!(res.is_hamiltonian());

        res
    }

    fn local_search_step(
        &self,
        candidate: &mut Route,
        neighborhood: &[usize],
        old_penalty_factor: f64,
        penalty_factor: f64,
    ) -> i32 {
        debug_assert_eq!(candidate.len(), neighborhood.len());

        for i in 0..neighborhood.len() {
            for j in neighborhood.iter().skip(i + 2).copied() {
                // Find vertexes on the route
                let i = neighborhood[i];
                let i_next = (i + 1) % candidate.len();
                let j_next = (j + 1) % candidate.len();

                // Find vertexes to twist
                let i_vertex = candidate[i];
                let i_vertex_next = candidate[i_next];

                let j_vertex = candidate[j];
                let j_vertex_next = candidate[j_next];

                // Calculate the new cost: {i, i+1}, {j, j+1} -> {i, j}, {i+1, j+1}
                let cost_change_decreased =
                    self[(i_vertex, i_vertex_next)]
                        + self[(j_vertex, j_vertex_next)]
                        + (
                        old_penalty_factor *
                            (
                                self.penalty((i_vertex, i_vertex_next))
                                    + self.penalty((j_vertex, j_vertex_next))
                            ) as f64
                    ) as i32;


                let cost_change_increased =
                    self[(i_vertex, j_vertex)]
                        + self[(i_vertex_next, j_vertex_next)]
                        + (
                        penalty_factor *
                            (
                                self.penalty((i_vertex, j_vertex))
                                    + self.penalty((i_vertex_next, j_vertex_next))
                            ) as f64
                    ) as i32;

                let cost_change = cost_change_increased - cost_change_decreased;

                // If the cost is decreased, apply the twist and finish the step
                if cost_change < 0 {
                    candidate.twist(i_next, j, cost_change);
                    return cost_change;
                }
            }
        }

        0
    }

    pub fn local_search(
        &self,
        candidate: &mut Route,
        neighborhood: &[usize],
        old_penalty_factor: f64,
        penalty_factor: f64)
    {
        loop {
            let change = self.local_search_step(candidate, &neighborhood, old_penalty_factor, penalty_factor);
            if change == 0 { break; }
        }
    }

    /// Solve the instance
    ///
    /// `seed`: seed used for the RNG.
    pub fn solve(&self, seed: u64) -> Route {
        // RNG
        let mut rng: Mt64 = SeedableRng::seed_from_u64(seed);

        // Neighborhood search
        let mut neighborhood: Vec<_> = (0..self.size).collect();
        neighborhood.shuffle(&mut rng);
        let neighborhood = neighborhood;

        let mut route = self.nearest_neighbor();

        self.local_search(&mut route, &neighborhood, 0.0, 0.0);
        //let penalty_factor = 0.3 * (route.cost as f64) / (route.len() as f64);

        // Find the maximum utility among all features
        let max_utility = route.edges()
            .map(|vertex| self[vertex] / (1 + self.penalty(vertex)))
            .max()
            .unwrap();

        println!("{}", max_utility);

        /*let v = self[(route[max], route[max + 1])] / (1 + self.penalty((route[max], route[max + 1])));
        println!("{:?} {} {} {}", max, route[max], route[max + 1], v);*/

        route
    }
}

impl Index<(usize, usize)> for GuidedLocalSearch {
    type Output = i32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = self.get_index(index);
        unsafe { self.data.get_unchecked(index) }
    }
}

impl IndexMut<(usize, usize)> for GuidedLocalSearch {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = self.get_index(index);
        unsafe { self.data.get_unchecked_mut(index) }
    }
}

impl Display for GuidedLocalSearch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let precision = f.precision().unwrap_or(self.size);

        write!(f, "GuidedLocalSearch: {{\n    size: {},\n    data (precision {}):\n", self.size, precision)?;

        write!(f, "              ")?;
        for i in 0..precision { write!(f, "{:>3} ", i)?; }
        write!(f, "\n             ")?;
        for _ in 0..precision { write!(f, "____")?; }
        writeln!(f)?;

        for i in 0..precision {
            write!(f, "        {:>3} | ", i)?;
            for j in 0..precision {
                write!(f, "{:>3} ", self[(i, j)])?;
            }
            writeln!(f)?;
        }

        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use crate::types::gls::GuidedLocalSearch;
    use crate::types::point::Point;

    fn create_graph() -> GuidedLocalSearch {
        let points = vec![
            Point::new(2.83000e+03 as i32, 4.00000e+01 as i32),
            Point::new(2.83000e+03 as i32, 7.70000e+01 as i32),
            Point::new(2.83000e+03 as i32, 1.14000e+02 as i32),
            Point::new(2.83100e+03 as i32, 1.55000e+02 as i32),
            Point::new(2.83000e+03 as i32, 1.94000e+02 as i32),
            Point::new(2.83100e+03 as i32, 2.31000e+02 as i32),
            Point::new(2.83100e+03 as i32, 2.69000e+02 as i32),
            Point::new(2.83100e+03 as i32, 3.09000e+02 as i32),
            Point::new(2.83000e+03 as i32, 3.47000e+02 as i32),
            Point::new(2.83000e+03 as i32, 3.84000e+02 as i32),
        ];
        GuidedLocalSearch::new(&points)
    }

    fn simple_graph() -> GuidedLocalSearch {
        GuidedLocalSearch::from_data(4, vec![
            0, 1, 2, 5,
            1, 0, 7, 4,
            2, 7, 0, 1,
            5, 4, 1, 0,
        ])
    }

    #[cfg(test)]
    mod create {
        use crate::types::gls::tests::create_graph;

        #[test]
        fn test() {
            let actual = create_graph();
            let expected = vec![
                0, 37, 74, 115, 154, 191, 229, 269, 307, 344,
                37, 0, 37, 78, 117, 154, 192, 232, 270, 307,
                74, 37, 0, 41, 80, 117, 155, 195, 233, 270,
                115, 78, 41, 0, 39, 76, 114, 154, 192, 229,
                154, 117, 80, 39, 0, 37, 75, 115, 153, 190,
                191, 154, 117, 76, 37, 0, 38, 78, 116, 153,
                229, 192, 155, 114, 75, 38, 0, 40, 78, 115,
                269, 232, 195, 154, 115, 78, 40, 0, 38, 75,
                307, 270, 233, 192, 153, 116, 78, 38, 0, 37,
                344, 307, 270, 229, 190, 153, 115, 75, 37, 0,
            ];
            assert_eq!(actual.data, expected)
        }
    }

    #[test]
    fn index_test() {
        let actual = create_graph();

        assert_eq!(actual[(0, 0)], 0);
        assert_eq!(actual[(0, 1)], 37);
        assert_eq!(actual[(0, 2)], 74);
        assert_eq!(actual[(0, 3)], 115);
        assert_eq!(actual[(0, 4)], 154);
        assert_eq!(actual[(0, 5)], 191);
        assert_eq!(actual[(0, 6)], 229);
        assert_eq!(actual[(0, 7)], 269);
        assert_eq!(actual[(0, 8)], 307);
        assert_eq!(actual[(0, 9)], 344);

        assert_eq!(actual[(1, 0)], 37);
        assert_eq!(actual[(1, 1)], 0);
        assert_eq!(actual[(1, 2)], 37);
        assert_eq!(actual[(1, 3)], 78);
        assert_eq!(actual[(1, 4)], 117);
        assert_eq!(actual[(1, 5)], 154);
        assert_eq!(actual[(1, 6)], 192);
        assert_eq!(actual[(1, 7)], 232);
        assert_eq!(actual[(1, 8)], 270);
        assert_eq!(actual[(1, 9)], 307);

        assert_eq!(actual[(2, 0)], 74);
        assert_eq!(actual[(2, 1)], 37);
        assert_eq!(actual[(2, 2)], 0);
        assert_eq!(actual[(2, 3)], 41);
        assert_eq!(actual[(2, 4)], 80);
        assert_eq!(actual[(2, 5)], 117);
        assert_eq!(actual[(2, 6)], 155);
        assert_eq!(actual[(2, 7)], 195);
        assert_eq!(actual[(2, 8)], 233);
        assert_eq!(actual[(2, 9)], 270);

        assert_eq!(actual[(3, 0)], 115);
        assert_eq!(actual[(3, 1)], 78);
        assert_eq!(actual[(3, 2)], 41);
        assert_eq!(actual[(3, 3)], 0);
        assert_eq!(actual[(3, 4)], 39);
        assert_eq!(actual[(3, 5)], 76);
        assert_eq!(actual[(3, 6)], 114);
        assert_eq!(actual[(3, 7)], 154);
        assert_eq!(actual[(3, 8)], 192);
        assert_eq!(actual[(3, 9)], 229);

        assert_eq!(actual[(4, 0)], 154);
        assert_eq!(actual[(4, 1)], 117);
        assert_eq!(actual[(4, 2)], 80);
        assert_eq!(actual[(4, 3)], 39);
        assert_eq!(actual[(4, 4)], 0);
        assert_eq!(actual[(4, 5)], 37);
        assert_eq!(actual[(4, 6)], 75);
        assert_eq!(actual[(4, 7)], 115);
        assert_eq!(actual[(4, 8)], 153);
        assert_eq!(actual[(4, 9)], 190);

        assert_eq!(actual[(5, 0)], 191);
        assert_eq!(actual[(5, 1)], 154);
        assert_eq!(actual[(5, 2)], 117);
        assert_eq!(actual[(5, 3)], 76);
        assert_eq!(actual[(5, 4)], 37);
        assert_eq!(actual[(5, 5)], 0);
        assert_eq!(actual[(5, 6)], 38);
        assert_eq!(actual[(5, 7)], 78);
        assert_eq!(actual[(5, 8)], 116);
        assert_eq!(actual[(5, 9)], 153);

        assert_eq!(actual[(6, 0)], 229);
        assert_eq!(actual[(6, 1)], 192);
        assert_eq!(actual[(6, 2)], 155);
        assert_eq!(actual[(6, 3)], 114);
        assert_eq!(actual[(6, 4)], 75);
        assert_eq!(actual[(6, 5)], 38);
        assert_eq!(actual[(6, 6)], 0);
        assert_eq!(actual[(6, 7)], 40);
        assert_eq!(actual[(6, 8)], 78);
        assert_eq!(actual[(6, 9)], 115);

        assert_eq!(actual[(7, 0)], 269);
        assert_eq!(actual[(7, 1)], 232);
        assert_eq!(actual[(7, 2)], 195);
        assert_eq!(actual[(7, 3)], 154);
        assert_eq!(actual[(7, 4)], 115);
        assert_eq!(actual[(7, 5)], 78);
        assert_eq!(actual[(7, 6)], 40);
        assert_eq!(actual[(7, 7)], 0);
        assert_eq!(actual[(7, 8)], 38);
        assert_eq!(actual[(7, 9)], 75);

        assert_eq!(actual[(8, 0)], 307);
        assert_eq!(actual[(8, 1)], 270);
        assert_eq!(actual[(8, 2)], 233);
        assert_eq!(actual[(8, 3)], 192);
        assert_eq!(actual[(8, 4)], 153);
        assert_eq!(actual[(8, 5)], 116);
        assert_eq!(actual[(8, 6)], 78);
        assert_eq!(actual[(8, 7)], 38);
        assert_eq!(actual[(8, 8)], 0);
        assert_eq!(actual[(8, 9)], 37);

        assert_eq!(actual[(9, 0)], 344);
        assert_eq!(actual[(9, 1)], 307);
        assert_eq!(actual[(9, 2)], 270);
        assert_eq!(actual[(9, 3)], 229);
        assert_eq!(actual[(9, 4)], 190);
        assert_eq!(actual[(9, 5)], 153);
        assert_eq!(actual[(9, 6)], 115);
        assert_eq!(actual[(9, 7)], 75);
        assert_eq!(actual[(9, 8)], 37);
        assert_eq!(actual[(9, 9)], 0);
    }

    #[cfg(test)]
    mod sum_edges {
        use crate::types::gls::tests::simple_graph;

        #[test]
        fn seq() {
            let graph = simple_graph();
            let sum = graph.sum_edges(&[0, 1, 2, 3]);
            assert_eq!(sum, 14);
        }

        #[test]
        fn alter() {
            let graph = simple_graph();
            let sum = graph.sum_edges(&[1, 3, 0, 2]);
            assert_eq!(sum, 18);
        }
    }
}
