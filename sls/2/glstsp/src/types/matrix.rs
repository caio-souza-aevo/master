use crate::types::point::Point;
use std::ops::Index;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Eq, PartialEq)]
pub struct SymmetricMatrix {
    size: usize,
    data: Vec<i32>,
}

impl SymmetricMatrix {
    pub fn from_size(size: usize) -> SymmetricMatrix {
        let data = vec![0i32; size * size];
        Self { size, data }
    }

    pub fn from_euclidean_coords(points: &[Point]) -> Self {
        let size = points.len();
        assert!(size > 0);

        let mut res = Self::from_size(size);

        for (i, point) in points.iter().copied().enumerate() {
            for (j, neighbor) in points.iter().copied().enumerate().skip(i + 1) {
                let dist = point.dist(neighbor);
                res.set(i, j, dist);
            }
        }

        res
    }

    #[inline]
    fn get_index(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.size);
        debug_assert!(y < self.size);
        x * self.size + y
    }

    pub fn set(&mut self, x: usize, y: usize, value: i32) {
        let ia = self.get_index(x, y);
        self.data[ia] = value;

        let ib = self.get_index(y, x);
        self.data[ib] = value;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn sum(&self, indexes: impl Iterator<Item=(usize, usize)>) -> i32 {
        indexes
            .map(|edge| self[edge])
            .sum()
    }
}

impl Index<(usize, usize)> for SymmetricMatrix {
    type Output = i32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        let index = self.get_index(x, y);
        unsafe { self.data.get_unchecked(index) }
    }
}

impl Display for SymmetricMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let precision = f.precision().unwrap_or(self.size);

        let precision_fmt = if precision == self.size {
            ":     ".to_owned()
        } else {
            format!(" (precision {}):\n              ", precision)
        };

        write!(f, "SymmetricMatrix: {{\n    size: {},\n    data{}", self.size, precision_fmt)?;

        for i in 0..precision { write!(f, "{:>3} ", i)?; }
        write!(f, "\n             ")?;
        for _ in 0..precision { write!(f, "----")?; }
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
    use crate::types::matrix::SymmetricMatrix;
    use crate::types::point::Point;

    fn create_matrix() -> SymmetricMatrix {
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
        SymmetricMatrix::from_euclidean_coords(&points)
    }

    fn simple_matrix() -> SymmetricMatrix {
        SymmetricMatrix {
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
        use crate::types::matrix::tests::create_matrix;

        #[test]
        fn test() {
            let actual = create_matrix();
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
        let actual = create_matrix();

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
    mod sum {
        use crate::types::matrix::tests::simple_matrix;

        #[test]
        fn seq() {
            let graph = simple_matrix();
            let indexes = vec![(0usize, 1usize), (1, 2), (2, 3), (3, 0)];
            let actual = graph.sum(indexes.into_iter());
            assert_eq!(actual, 14);
        }

        #[test]
        fn alter() {
            let graph = simple_matrix();
            let indexes = vec![(1usize, 3usize), (3, 0), (0, 2), (2, 1)];
            let actual = graph.sum(indexes.into_iter());
            assert_eq!(actual, 18);
        }
    }
}
