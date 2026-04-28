use std::{ops::{Add, Sub, Neg, Mul, Div}};
fn tick(env: (Vector,Vector), projectile: (Vector,Vector) )-> (Vector,Vector){
    let pos = projectile.0 + projectile.1;
    let v = projectile.1 + env.0+env.1;
    (pos,v)
}
fn main() {
    println!("Hello, world!");
    let env = (Vector::new(0.0,-0.1,0.0),Vector::new(-0.01,0.0,0.0));
    let mut projectile = (Vector::new(0.0,1.0,0.0),normalize(Vector::new(1.0,1.0,0.0)));
    while projectile.0.0.y >0.0{
        projectile=tick(env, projectile);
        dbg!(projectile);

    }
}

#[derive(PartialEq, Debug,Copy,Clone)]
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
impl Tuple{
    fn new(x:f64,y:f64,z:f64,w:f64) -> Self{
        return Tuple{x,y,z,w}
    }
}
#[derive(PartialEq, Debug,Copy,Clone)]
struct Point(Tuple);
impl Point{
    fn new(x:f64,y:f64,z:f64) -> Self {
        Point(Tuple{x,y,z,w:1.0})
    }
}
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
#[derive(PartialEq, Debug,Copy,Clone)]
struct Vector(Tuple);
impl Vector{
    fn new(x:f64,y:f64,z:f64)-> Self{
        Vector(Tuple{x,y,z,w:0.0})
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
// Operations
impl Add for Tuple {
    type Output = Tuple;
    fn add(self, rhs: Self) -> Tuple {
        Tuple{
        x: self.x + rhs.x,
        y: self.y + rhs.y,
        z: self.z + rhs.z,
        w: self.w + rhs.w
        }
        
    }
}
impl Add<Point> for Vector {
    type Output = Point;
    fn add(self,other: Point) -> Point{
    Point(self.0+other.0)
    }
}
impl Add<Vector> for Point {
    type Output = Point;
    fn add(self,other: Vector) -> Point{
    Point(self.0+other.0)
    }
}
impl Add for Vector{
    type Output= Vector;
    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0+rhs.0)
    }
}
impl Sub for Tuple{
    type Output = Tuple;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple{
            x: self.x-rhs.x,
            y: self.y-rhs.y,
            z: self.z-rhs.z,
            w: self.w-rhs.w,
        }
    }
} 
impl Sub for Point{
    type Output = Vector;
    fn sub(self,rhs:Self) -> Self::Output{
        Vector(self.0-rhs.0)
    }
}
impl Sub<Vector> for Point{
    type Output = Point;
    fn sub(self,rhs:Vector) -> Point{
        Point(self.0-rhs.0)
    }
}
impl Sub for Vector{
    type Output = Vector;
    fn sub(self,rhs:Self) -> Self::Output{
        Vector(self.0-rhs.0)
    }
}
impl Neg for Tuple{
    type Output = Tuple;
    fn neg(self) -> Self::Output{
            Tuple{ 
                x: -self.x,
                y: -self.y,
                z: -self.z,
                w: -self.w,
            }
        }
    }
impl Neg for Vector{
    type Output = Vector;
    fn neg(self)->Self::Output{
        Vector(-self.0)
    }
}
impl Mul<f64> for Tuple{
    type Output = Tuple;
    fn mul(self, rhs:f64 )-> Self::Output{
        Tuple{
            x: self.x*rhs,
            y: self.y*rhs,
            z: self.z*rhs,
            w: self.w*rhs,
        }

    }
}
impl Div<f64> for Tuple{
    type Output = Tuple;
    fn div(self, rhs:f64 )-> Self::Output{
        Tuple{
            x: self.x/rhs,
            y: self.y/rhs,
            z: self.z/rhs,
            w: self.w/rhs,
        }

    }
}
fn magnitude(v: Vector)->f64{
    (v.0.x.powi(2)+v.0.y.powi(2)+v.0.z.powi(2)).sqrt()
}
fn normalize(v: Vector)-> Vector{
    let m =magnitude(v);
    Vector::new(v.0.x/m,v.0.y/m,v.0.z/m)

}
fn dot(lhs:Vector, rhs:Vector) -> f64{
    lhs.0.x*rhs.0.x+lhs.0.y*rhs.0.y+lhs.0.z*rhs.0.z
}
fn cross(lhs:Vector,rhs:Vector)->Vector{
    Vector::new(
        lhs.0.y*rhs.0.z - lhs.0.z*rhs.0.y,
        lhs.0.z*rhs.0.x - lhs.0.x*rhs.0.z,
        lhs.0.x*rhs.0.y - lhs.0.y*rhs.0.x,
    )
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
//  	​Scenario​: Adding two tuples
// ​ 	  ​Given​ a1 ← tuple(3, -2, 5, 1)
// ​ 	    ​And​ a2 ← tuple(-2, 3, 1, 0)
// ​ 	   ​Then​ a1 + a2 = tuple(1, 1, 6, 1)
    #[test]
    fn add_tuple(){
        let a1: Tuple = (3.0,-2.0,5.0,1.0).into();
        let a2: Tuple = (-2.0,3.0,1.0,0.0).into();
        assert_eq!(a1+a2,Tuple{x: 1.0,y: 1.0,z: 6.0,w: 1.0})
    }
// 	​Scenario​: Subtracting two points
// ​ 	  ​Given​ p1 ← point(3, 2, 1)
// ​ 	    ​And​ p2 ← point(5, 6, 7)
// ​ 	  ​Then​ p1 - p2 = vector(-2, -4, -6)
    #[test]
    fn sub_points(){
        let p1 :Point = (3.0,2.0,1.0).into();
        let p2 :Point = (5.0,6.0,7.0).into();
        assert_eq!(p1-p2,Vector::new(-2.0,-4.0,-6.0));

    }
// 	​Scenario​: Subtracting a vector from a point
// ​ 	  ​Given​ p ← point(3, 2, 1)
// ​ 	    ​And​ v ← vector(5, 6, 7)
// ​ 	  ​Then​ p - v = point(-2, -4, -6)
    #[test]
    fn sub_point_vector(){
        let p :Point = (3.0,2.0,1.0).into();
        let v :Vector = (5.0,6.0,7.0).into();
        assert_eq!(p-v,Point::new(-2.0,-4.0,-6.0));
    }
// Scenario​: Subtracting two vectors
// ​ 	  ​Given​ v1 ← vector(3, 2, 1)
// ​ 	    ​And​ v2 ← vector(5, 6, 7)
// ​ 	  ​Then​ v1 - v2 = vector(-2, -4, -6)
    #[test]
    fn sub_vectors(){

        let v1 :Vector = (3.0,2.0,1.0).into();
        let v2 :Vector = (5.0,6.0,7.0).into();
        assert_eq!(v1-v2,Vector::new(-2.0,-4.0,-6.0));
    }
// ​ 	​Scenario​: Subtracting a vector from the zero vector
// ​ 	  ​Given​ zero ← vector(0, 0, 0)
// ​ 	    ​And​ v ← vector(1, -2, 3)
// ​ 	  ​Then​ zero - v = vector(-1, 2, -3)
    #[test]
    fn sub_vectors_zero(){
        let v1 :Vector = (0.0,0.0,0.0).into();
        let v2 :Vector = (1.0,-2.0,3.0).into();
        assert_eq!(v1-v2,Vector::new(-1.0,2.0,-3.0));

    }
// ​ 	​Scenario​: Negating a tuple
// ​ 	  ​Given​ a ← tuple(1, -2, 3, -4)
// ​ 	  ​Then​ -a = tuple(-1, 2, -3, 4)
    #[test]
    fn negate_tuple(){
        let a: Tuple = (1.0,-2.0,3.0,-4.0).into();
        assert_eq!(-a,Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }
    #[test]
    fn negate_vector(){
        let v= Vector::new(1.5, -2.0, 3.5);
        assert_eq!(-v,Vector::new(-1.5, 2.0, -3.5))
    }
// 	​Scenario​: Multiplying a tuple by a scalar
// ​ 	  ​Given​ a ← tuple(1, -2, 3, -4)
// ​ 	  ​Then​ a * 3.5 = tuple(3.5, -7, 10.5, -14)
// ​ 	
// ​ 	​Scenario​: Multiplying a tuple by a fraction
// ​ 	  ​Given​ a ← tuple(1, -2, 3, -4)
// ​ 	  ​Then​ a * 0.5 = tuple(0.5, -1, 1.5, -2)
    #[test]
    fn scale_tuple(){
        let a = Tuple::new(1.0,-2.0,3.0,-4.0);
        let scaled_a1 = a* 3.5;
        let scaled_a2 = a *0.5;
        assert_eq!(scaled_a1,Tuple::new(3.5,-7.0,10.5,-14.0));
        assert_eq!(scaled_a2,Tuple::new(0.5,-1.0,1.5,-2.0));
    }
// ​ 	​Scenario​: Dividing a tuple by a scalar
// ​ 	  ​Given​ a ← tuple(1, -2, 3, -4)
// ​ 	  ​Then​ a / 2 = tuple(0.5, -1, 1.5, -2)
    #[test]
    fn divide_tuple(){
        let a = Tuple::new(1.0,-2.0,3.0,-4.0);
        let d = a/2.0;
        assert_eq!(d,Tuple::new(0.5,-1.0,1.5,-2.0));

    }
//  	​Scenario​: Computing the magnitude of vector(1, 0, 0)
// ​ 	  ​Given​ v ← vector(1, 0, 0)
// ​ 	  ​Then​ magnitude(v) = 1
// ​ 	
// ​ 	​Scenario​: Computing the magnitude of vector(0, 1, 0)
// ​ 	  ​Given​ v ← vector(0, 1, 0)
// ​ 	  ​Then​ magnitude(v) = 1
// ​ 	
// ​ 	​Scenario​: Computing the magnitude of vector(0, 0, 1)
// ​ 	  ​Given​ v ← vector(0, 0, 1)
// ​ 	  ​Then​ magnitude(v) = 1
// ​ 	
// ​ 	​Scenario​: Computing the magnitude of vector(1, 2, 3)
// ​ 	  ​Given​ v ← vector(1, 2, 3)
// ​ 	  ​Then​ magnitude(v) = √14
// ​ 	
// ​ 	​Scenario​: Computing the magnitude of vector(-1, -2, -3)
// ​ 	  ​Given​ v ← vector(-1, -2, -3)
// ​ 	  ​Then​ magnitude(v) = √14
    #[test]
    fn test_mag(){
        let v1 = Vector::new(1.0,0.0,0.0);
        assert_eq!(magnitude(v1),1.0);
        let v2 = Vector::new(0.0,1.0,0.0);
        assert_eq!(magnitude(v2),1.0);
        let v3 = Vector::new(0.0,0.0,1.0);
        assert_eq!(magnitude(v3),1.0);
        let v4 = Vector::new(1.0,2.0,3.0);
        assert_eq!(magnitude(v4),14_f64.sqrt());
        let v5 = Vector::new(-1.0,-2.0,-3.0);
        assert_eq!(magnitude(v5),14_f64.sqrt());

    }
//     Scenario​: Normalizing vector(4, 0, 0) gives (1, 0, 0)
// ​ 	  ​Given​ v ← vector(4, 0, 0)
// ​ 	  ​Then​ normalize(v) = vector(1, 0, 0)
// ​ 	
// ​ 	​Scenario​: Normalizing vector(1, 2, 3)
// ​ 	  ​Given​ v ← vector(1, 2, 3)
// ​ 	                                  ​# vector(1/√14,   2/√14,   3/√14)​
// ​ 	  ​Then​ normalize(v) = approximately vector(0.26726, 0.53452, 0.80178)
// ​ 	
// ​ 	​Scenario​: The magnitude of a normalized vector
// ​ 	  ​Given​ v ← vector(1, 2, 3)
// ​ 	  ​When​ norm ← normalize(v)
// ​ 	  ​Then​ magnitude(norm) = 1
    #[test]
    fn test_norm(){
        let v1 = Vector::new(4.0,0.0,0.0);
        let n1 = normalize(v1);
        assert_eq!(n1,Vector::new(1.0,0.0,0.0));
        let v2 = Vector::new(1.0,2.0,3.0);
        let n2 = normalize(v2);
        assert_eq!(n2,Vector::new(1.0/(14.0_f64.sqrt()), 2.0/(14.0_f64.sqrt()), 3.0/(14.0_f64.sqrt()) ));
        assert_eq!(magnitude(n2),1.0)

    }
//     ​Scenario​: The dot product of two tuples
// ​ 	  ​Given​ a ← vector(1, 2, 3)
// ​ 	    ​And​ b ← vector(2, 3, 4)
// ​ 	  ​Then​ dot(a, b) = 20
    #[test]
    fn test_dot_prod(){
        let v1 = Vector::new(1.0,2.0,3.0);
        let v2 = Vector::new(2.0,3.0,4.0);
        assert_eq!(dot(v1,v2),20.0)

    }
// ​ 	​Scenario​: The cross product of two vectors
// ​ 	  ​Given​ a ← vector(1, 2, 3)
// ​ 	    ​And​ b ← vector(2, 3, 4)
// ​ 	  ​Then​ cross(a, b) = vector(-1, 2, -1)
// ​ 	    ​And​ cross(b, a) = vector(1, -2, 1)
    #[test]
    fn test_cross_prod(){
        let v1 = Vector::new(1.0,2.0,3.0);
        let v2 = Vector::new(2.0,3.0,4.0);
        assert_eq!(cross(v1,v2),Vector::new(-1.0,2.0,-1.0));
        assert_eq!(cross(v2,v1),Vector::new(1.0,-2.0,1.0));
    }
}