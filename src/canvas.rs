use crate::color::{Color, ppm_format};
pub struct Canvas {
    width : usize,
    height : usize,
    pixels : Vec<Color>
}
impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas{
            width: width,
            height: height,
            pixels: vec![Color::new(0.0,0.0,0.0);width*height],
        }
    }
}
pub fn write_pixel(c: &mut Canvas, x: usize, y: usize, color: Color) {
   c.pixels[y*c.width+x] = color
}
fn pixel_at(c :&Canvas, x: usize,y:usize)->Color{
    c.pixels[y*c.width+x]
}
pub fn canvas_to_ppm(c: &Canvas) -> String {
    let mut s = String::new();
    s.push_str("P3\n");
    s.push_str(&format!("{} {}\n",c.width,c.height));
    s.push_str("255\n");

    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut x: usize =0;
    for co in &c.pixels {
        let s = ppm_format(*co);
        x=x+1;
        if current.is_empty() {
            current = s;
        } else if x > c.width {
            lines.push(current);
            current = s;
            x=1;
        } else if current.len() + 1 + s.len() <= 70 {
            current.push(' ');
            current.push_str(&s);
        } else {
            let mut rest = String::new();
            let mut overflow = false;
            for (i, value) in s.split(" ").enumerate() {
                if !overflow && current.len() + 1 + value.len() <= 70 {
                    current.push(' ');
                    current.push_str(value);
                } else {
                    overflow = true;
                    rest.push_str(value);
                    if i != 2 {
                        rest.push(' ');
                    }
                }
            }
            lines.push(current);
            current = rest;
             

        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    let body = lines.join("\n")+"\n";
    s.push_str(&body);
    s
}

#[cfg(test)]
mod tests {
    use super::*;
// 	​Scenario​: Creating a canvas
// ​ 	  ​Given​ c ← canvas(10, 20)
// ​ 	  ​Then​ c.width = 10
// ​ 	    ​And​ c.height = 20
// ​ 	    ​And​ every pixel of c is color(0, 0, 0)
//  	​Scenario​: Writing pixels to a canvas
// ​ 	  ​Given​ c ← canvas(10, 20)
// ​ 	    ​And​ red ← color(1, 0, 0)
// ​ 	  ​When​ write_pixel(c, 2, 3, red)
// ​ 	  ​Then​ pixel_at(c, 2, 3) = red
    #[test]
    fn test_blank(){
        let c = Canvas::new(10,20);
        assert_eq!(c.height,20);
        assert_eq!(c.width,10);
        let zero=Color::new(0.0,0.0,0.0);
        for co in c.pixels{
            assert_eq!(co,zero)
        }
    }
    #[test]
    fn test_write_pixel(){
        let mut c = Canvas::new(10,20);
        let red = Color::new(1.0,0.0,0.0);
        write_pixel(&mut c,2,3,red);
        assert_eq!(pixel_at(&c,2,3),red);
        

    }
    #[test]
    fn test_ppm_header(){
// 	​Scenario​: Constructing the PPM header
// ​ 	  ​Given​ c ← canvas(5, 3)
// ​ 	  ​When​ ppm ← canvas_to_ppm(c)
// ​ 	  ​Then​ lines 1-3 of ppm are
// ​ 	    ​"""​
// ​ 	​    P3​
// ​ 	​    5 3​
// ​ 	​    255​
// ​ 	​    """
        let c = Canvas::new(5,3);
        let s = canvas_to_ppm(&c);
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines[0],"P3");
        assert_eq!(lines[1],"5 3");
        assert_eq!(lines[2],"255")
    }
    #[test]
    fn test_ppm_body(){
//  	​Scenario​: Constructing the PPM pixel data
// ​ 	  ​Given​ c ← canvas(5, 3)
// ​ 	    ​And​ c1 ← color(1.5, 0, 0)
// ​ 	    ​And​ c2 ← color(0, 0.5, 0)
// ​ 	    ​And​ c3 ← color(-0.5, 0, 1)
// ​ 	  ​When​ write_pixel(c, 0, 0, c1)
// ​ 	    ​And​ write_pixel(c, 2, 1, c2)
// ​ 	    ​And​ write_pixel(c, 4, 2, c3)
// ​ 	    ​And​ ppm ← canvas_to_ppm(c)
// ​ 	  ​Then​ lines 4-6 of ppm are
// ​ 	    ​"""​
// ​ 	​    255 0 0 0 0 0 0 0 0 0 0 0 0 0 0​
// ​ 	​    0 0 0 0 0 0 0 128 0 0 0 0 0 0 0​
// ​ 	​    0 0 0 0 0 0 0 0 0 0 0 0 0 0 255​
// ​ 	​    """
        let mut c = Canvas::new(5,3);
        let c1 = Color::new(1.5,0.0,0.0);
        let c2 = Color::new(0.0,0.5,0.0);
        let c3 = Color::new(-0.5,0.0,1.0);
        write_pixel(&mut c, 0, 0, c1);
        write_pixel(&mut c, 2, 1, c2);
        write_pixel(&mut c, 4, 2, c3);
        let s = canvas_to_ppm(&c);
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines[3],"255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4],"0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5],"0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }
    #[test]
    fn test_long_line(){
// Scenario​: Splitting long lines in PPM files
// ​ 	  ​Given​ c ← canvas(10, 2)
// ​ 	  ​When​ every pixel of c is set to color(1, 0.8, 0.6)
// ​ 	    ​And​ ppm ← canvas_to_ppm(c)
// ​ 	  ​Then​ lines 4-7 of ppm are
// ​ 	    ​"""​
// ​ 	​    255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
// ​ 	​    153 255 204 153 255 204 153 255 204 153 255 204 153
// ​ 	​    255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
// ​ 	​    153 255 204 153 255 204 153 255 204 153 255 204 153
// ​ 	​    """
        let mut c = Canvas::new(10,2);
        let c1=Color::new(1.0,0.8,0.6);
        for co in &mut c.pixels{
            *co = c1;
            
        }
        let s = canvas_to_ppm(&c);
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines[3],"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
        assert_eq!(lines[4],"153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert_eq!(lines[5],"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
        assert_eq!(lines[6],"153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert!(s.ends_with('\n'));
    }
}
