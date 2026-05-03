use std::ops::{Add, Sub, Neg, Mul, Div};
use crate::tuple::{Tuple, is_vector};
use crate::point::Point;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vector(pub Tuple);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector(Tuple { x, y, z, w: 0.0 })
    }

    pub fn magnitude(&self) -> f64 {
        (self.0.x.powi(2) + self.0.y.powi(2) + self.0.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let m = self.magnitude();
        assert!(m > f64::EPSILON);
        Vector::new(self.0.x / m, self.0.y / m, self.0.z / m)
    }

    pub fn dot(&self, rhs: Vector) -> f64 {
        self.0.x * rhs.0.x + self.0.y * rhs.0.y + self.0.z * rhs.0.z
    }

    pub fn cross(&self, rhs: Vector) -> Vector {
        Vector::new(
            self.0.y * rhs.0.z - self.0.z * rhs.0.y,
            self.0.z * rhs.0.x - self.0.x * rhs.0.z,
            self.0.x * rhs.0.y - self.0.y * rhs.0.x,
        )
    }
}

impl TryFrom<Tuple> for Vector {
    type Error = &'static str;
    fn try_from(t: Tuple) -> Result<Self, Self::Error> {
        if is_vector(&t) {
            Ok(Vector(t))
        } else {
            Err("Tuple is not a vector")
        }
    }
}

impl From<(f64, f64, f64)> for Vector {
    fn from(tup: (f64, f64, f64)) -> Self {
        Vector(Tuple::from((tup.0, tup.1, tup.2, 0.0)))
    }
}

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0)
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0)
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector(-self.0)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector(self.0 * rhs)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        Vector(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn vector_compare_tuple() {
        let v: Vector = (4.0, -4.0, 3.0).into();
        let t: Tuple = (4.0, -4.0, 3.0, 0.0).into();
        assert_eq!(v.0, t)
    }

    #[test]
    fn sub_vectors() {
        let v1: Vector = (3.0, 2.0, 1.0).into();
        let v2: Vector = (5.0, 6.0, 7.0).into();
        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn sub_vectors_zero() {
        let v1: Vector = (0.0, 0.0, 0.0).into();
        let v2: Vector = (1.0, -2.0, 3.0).into();
        assert_eq!(v1 - v2, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negate_vector() {
        let v = Vector::new(1.5, -2.0, 3.5);
        assert_eq!(-v, Vector::new(-1.5, 2.0, -3.5))
    }

    #[test]
    fn test_mag() {
        assert_eq!(Vector::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Vector::new(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Vector::new(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(Vector::new(1.0, 2.0, 3.0).magnitude(), 14_f64.sqrt());
        assert_eq!(Vector::new(-1.0, -2.0, -3.0).magnitude(), 14_f64.sqrt());
    }

    #[test]
    fn test_norm() {
        assert_eq!(Vector::new(4.0, 0.0, 0.0).normalize(), Vector::new(1.0, 0.0, 0.0));
        let n2 = Vector::new(1.0, 2.0, 3.0).normalize();
        let s = 14.0_f64.sqrt();
        assert_eq!(n2, Vector::new(1.0 / s, 2.0 / s, 3.0 / s));
        assert_eq!(n2.magnitude(), 1.0);
    }

    #[test]
    fn test_dot_prod() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(v2), 20.0)
    }

    #[test]
    fn scale_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(v * 2.0, Vector::new(2.0, -4.0, 6.0));
        assert_eq!(v * 0.5, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn divide_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(v / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn test_cross_prod() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
    }
}
