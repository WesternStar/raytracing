use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }
}

pub fn is_vector(t: &Tuple) -> bool {
    t.w == 0.0
}

pub fn is_point(t: &Tuple) -> bool {
    t.w == 1.0
}

impl From<(f64, f64, f64, f64)> for Tuple {
    fn from(tup: (f64, f64, f64, f64)) -> Self {
        Tuple { x: tup.0, y: tup.1, z: tup.2, w: tup.3 }
    }
}

impl Add for Tuple {
    type Output = Tuple;
    fn add(self, rhs: Self) -> Tuple {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w1_is_a_point() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(is_point(&a));
        assert!(!is_vector(&a));
    }

    #[test]
    fn tuple_with_w0_is_a_vector() {
        let b: Tuple = (4.3, -4.2, 3.1, 0.0).into();
        assert_eq!(b.x, 4.3);
        assert_eq!(b.y, -4.2);
        assert_eq!(b.z, 3.1);
        assert_eq!(b.w, 0.0);
        assert!(!is_point(&b));
        assert!(is_vector(&b));
    }

    #[test]
    fn add_tuple() {
        let a1: Tuple = (3.0, -2.0, 5.0, 1.0).into();
        let a2: Tuple = (-2.0, 3.0, 1.0, 0.0).into();
        assert_eq!(a1 + a2, Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0 })
    }

    #[test]
    fn negate_tuple() {
        let a: Tuple = (1.0, -2.0, 3.0, -4.0).into();
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn scale_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn divide_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }
}
