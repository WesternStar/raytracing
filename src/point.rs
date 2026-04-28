use std::ops::{Add, Sub};
use crate::tuple::{Tuple, is_point};
use crate::vector::Vector;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point(pub Tuple);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point(Tuple { x, y, z, w: 1.0 })
    }
}

impl From<Tuple> for Point {
    fn from(t: Tuple) -> Self {
        if is_point(&t) {
            Point(t)
        } else {
            panic!("Tuple is not a point")
        }
    }
}

impl From<(f64, f64, f64)> for Point {
    fn from(tup: (f64, f64, f64)) -> Self {
        Point(Tuple::from((tup.0, tup.1, tup.2, 1.0)))
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, other: Vector) -> Point {
        Point(self.0 + other.0)
    }
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Point {
        Point(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;
    use crate::vector::Vector;

    #[test]
    fn point_compare_tuple() {
        let p: Point = (4.0, -4.0, 3.0).into();
        let t: Tuple = (4.0, -4.0, 3.0, 1.0).into();
        assert_eq!(p.0, t)
    }

    #[test]
    fn sub_points() {
        let p1: Point = (3.0, 2.0, 1.0).into();
        let p2: Point = (5.0, 6.0, 7.0).into();
        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn sub_point_vector() {
        let p: Point = (3.0, 2.0, 1.0).into();
        let v: Vector = (5.0, 6.0, 7.0).into();
        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }
}
