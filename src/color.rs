use std::ops::{Add, Sub, Mul};

const EPSILON: f64 = 0.0001;

trait ApproxEq {
    fn approx_eq(&self, other: &Self) -> bool;
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Color{
    red:f64,
    green: f64,
    blue: f64,
}

impl From<(f64,f64,f64)> for Color{
    fn from(t:(f64,f64,f64))->Self{
    Color{
        red: t.0,
        green: t.1,
        blue:t.2,
    }

    }
}
impl Color{
    pub fn new(red:f64,green:f64,blue:f64)-> Self{
        Color{red,green,blue}
    }
}
impl Add for Color{
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color{
            red:self.red+rhs.red,
            blue: self.blue+rhs.blue,
            green: self.green+rhs.green,
        }
    }
}
impl Sub for Color{
    type Output =  Color;
    fn sub(self, rhs: Self) -> Self::Output {
        Color{
            red: self.red-rhs.red,
            blue: self.blue-rhs.blue,
            green: self.green-rhs.green
        }
    }
}
impl Mul for Color{
    type Output = Color;
    fn mul(self, rhs: Self) -> Self::Output {
        Color{
        red:self.red*rhs.red,
        blue:self.blue*rhs.blue,
        green:self.green*rhs.green,
        }
    }
}
impl ApproxEq for Color {
    fn approx_eq(&self, other: &Self) -> bool {
        (self.red - other.red).abs() < EPSILON
            && (self.green - other.green).abs() < EPSILON
            && (self.blue - other.blue).abs() < EPSILON
    }
}

impl Mul<f64> for Color{
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color{
            red: self.red*rhs,
            blue: self.blue*rhs,
            green:self.green*rhs
        }
    }
}
pub fn ppm_format(c: Color)->String{
    let red = (c.red*255.0).round() as u8;
    let green =(c.green*255.0).round() as u8;
    let blue = (c.blue*255.0).round() as u8;
    format!("{} {} {}",red,green,blue)
}
#[cfg(test)]
mod tests {
    use crate::color::{Color, ApproxEq};

// 	​Scenario​: Colors are (red, green, blue) tuples
// ​ 	  ​Given​ c ← color(-0.5, 0.4, 1.7)
// ​ 	  ​Then​ c.red = -0.5
// ​ 	    ​And​ c.green = 0.4
// ​ 	    ​And​ c.blue = 1.7
    #[test]
    fn test_color(){
        let c :Color = (-0.5,0.4,1.7).into();
        assert_eq!(c,Color::new(-0.5,0.4,1.7));
        assert_eq!(c.red,-0.5);
        assert_eq!(c.green,0.4);
        assert_eq!(c.blue,1.7);

    }
//  	​Scenario​: Adding colors
// ​ 	  ​Given​ c1 ← color(0.9, 0.6, 0.75)
// ​ 	    ​And​ c2 ← color(0.7, 0.1, 0.25)
// ​ 	   ​Then​ c1 + c2 = color(1.6, 0.7, 1.0)
// ​ 	
// ​ 	​Scenario​: Subtracting colors
// ​ 	  ​Given​ c1 ← color(0.9, 0.6, 0.75)
// ​ 	    ​And​ c2 ← color(0.7, 0.1, 0.25)
// ​ 	   ​Then​ c1 - c2 = color(0.2, 0.5, 0.5)
// ​ 	
// ​ 	​Scenario​: Multiplying a color by a scalar
// ​ 	  ​Given​ c ← color(0.2, 0.3, 0.4)
// ​ 	  ​Then​ c * 2 = color(0.4, 0.6, 0.8)
// ​ 	​Scenario​: Multiplying colors
// ​ 	  ​Given​ c1 ← color(1, 0.2, 0.4)
// ​ 	    ​And​ c2 ← color(0.9, 1, 0.1)
// ​ 	   ​Then​ c1 * c2 = color(0.9, 0.2, 0.04)
    #[test]
    fn test_ops(){
        let c1 = Color::new(0.9,0.6,0.75);
        let c2 = Color::new(0.7,0.1,0.25);
        assert!((c1+c2).approx_eq(&Color::new(1.6,0.7,1.0)));
        assert!((c1-c2).approx_eq(&Color::new(0.2,0.5,0.5)));
        let c3 = Color::new(0.2,0.3,0.4);
        assert!((c3*2.0).approx_eq(&Color::new(0.4,0.6,0.8)));
        let c4 = Color::new(1.0,0.2,0.4);
        let c5 = Color::new(0.9,1.0,0.1);
        assert!((c4*c5).approx_eq(&Color::new(0.9,0.2,0.04)))

    }

}