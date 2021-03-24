use std::ops::{Index, IndexMut};
use crate::types::point::Point;

#[derive(Eq, PartialEq, Debug)]
pub struct Route {
    pub path: Vec<usize>,
    pub cost: i32,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Graph {
    size: usize,
    data: Vec<i32>,
}

impl Graph {
    pub fn new(points: &[Point]) -> Self {
        let size = points.len();
        assert!(size > 0);

        let mut res = Self {
            size,
            data: vec![0i32; size * size],
        };

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
        x * self.size + y
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

    pub fn gls(&self, _seed: u64) -> Route {
        let path: Vec<_> = (0..self.size).collect();
        let cost = self.sum_edges(&path);
        Route { path, cost }
    }
}

impl Index<(usize, usize)> for Graph {
    type Output = i32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = self.get_index(index);

        #[cfg(debug_assertions)]
            { self.data.get(index).unwrap() }

        #[cfg(not(debug_assertions))]
            unsafe { self.data.get_unchecked(index) }
    }
}

impl IndexMut<(usize, usize)> for Graph {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = self.get_index(index);

        #[cfg(debug_assertions)]
            { self.data.get_mut(index).unwrap() }

        #[cfg(not(debug_assertions))]
            unsafe { self.data.get_unchecked_mut(index) }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::graph::Graph;
    use crate::types::point::Point;

    fn create_graph() -> Graph {
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
        Graph::new(&points)
    }

    fn simple_graph() -> Graph {
        Graph {
            size: 4,
            data: vec![
                0, 1, 2, 5,
                1, 0, 7, 4,
                2, 7, 0, 1,
                5, 4, 1, 0,
            ],
        }
    }

    #[cfg(test)]
    mod create {
        use crate::types::graph::tests::create_graph;

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
        use crate::types::graph::tests::simple_graph;

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