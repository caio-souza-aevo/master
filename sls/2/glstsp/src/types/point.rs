#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[cfg(test)]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn dist(self, other: Self) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let res = f64::sqrt(((dx * dx) + (dy * dy)) as f64);
        res as i32
    }
}

impl From<&str> for Point {
    fn from(str: &str) -> Self {
        let data = str.split(' ').collect::<Vec<_>>();
        let parse = |i: usize| {
            data[i].parse::<f32>().unwrap()
        } as i32;
        let x = parse(0);
        let y = parse(1);
        Point { x, y }
    }
}

#[cfg(test)]
mod tests_point {
    use crate::types::point::Point;

    #[test]
    fn from_string_0_0() {
        let expected = Point::new(0, 0);
        assert_eq!(Point::from("0 0"), expected);
        assert_eq!(Point::from("0e10 0e20"), expected);
        assert_eq!(Point::from("0.0e10 0.0e20"), expected);
    }

    #[test]
    fn from_string_1_2() {
        let expected = Point::new(1, 2);
        assert_eq!(Point::from("1 2"), expected);
        assert_eq!(Point::from("1e0 2e0"), expected);
        assert_eq!(Point::from("1.0e0 2.0e0"), expected);
    }

    #[test]
    fn from_string_10_20() {
        let expected = Point::new(10, 20);
        assert_eq!(Point::from("10 20"), expected);
        assert_eq!(Point::from("1e1 2e1"), expected);
        assert_eq!(Point::from("1.0e1 2.0e1"), expected);
    }
}
