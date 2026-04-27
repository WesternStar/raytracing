fn main() {
    println!("Hello, world!");
}
#[derive(PartialEq, Debug)]
struct Tuple{
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
impl From<(f64,f64,f64,f64)> for Tuple{
    fn from(tup:(f64,f64,f64,f64))->Self{
        Tuple{ x: tup.0, y: tup.1 , z: tup.2, w: tup.3}
    }
}
fn is_vector(t: &Tuple) -> bool{
    t.w == 0.0
}
fn is_point(t: &Tuple) -> bool{
    t.w == 1.0
}
struct Point(Tuple);
struct Vector(Tuple);
impl From<Tuple> for Point{
    fn from(t: Tuple) -> Self{
        if is_point(&t){
            Point(t)
        }else{
            panic!("Tuple is not a point")
        }
    }
}
impl From<(f64,f64,f64)> for Point{
    fn from(tup:(f64,f64,f64))->Self{
        Point(Tuple::from((tup.0,tup.1,tup.2,1.0)))
    }
}
impl From<Tuple> for Vector{
    fn from(t: Tuple) -> Self{
        if is_vector(&t){
            Vector(t)
        }else{
            panic!("Tuple is not a vector")
        }
    }
}
impl From<(f64,f64,f64)> for Vector{
    fn from(tup:(f64,f64,f64))->Self{
        Vector(Tuple::from((tup.0,tup.1,tup.2,0.0)))
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
// ​ 	​Scenario​: point() creates Tuples with w=1
// ​ 	  ​Given​ p ← point(4, -4, 3)
// ​ 	  ​Then​ p = Tuple(4, -4, 3, 1)
    #[test]
    fn point_compare_tuple() {
        let p: Point = (4.0, -4.0, 3.0).into();
        let t: Tuple = (4.0, -4.0, 3.0, 1.0).into();
        assert_eq!(p.0,t)
    }

// ​ 	​Scenario​: vector() creates Tuples with w=0
// ​ 	  ​Given​ v ← vector(4, -4, 3)
// ​ 	  ​Then​ v = Tuple(4, -4, 3, 0)
    #[test]
    fn vector_compare_tuple() {
        let v: Vector = (4.0, -4.0, 3.0).into();
        let t: Tuple = (4.0, -4.0, 3.0, 0.0).into();
        assert_eq!(v.0,t)
    }
}