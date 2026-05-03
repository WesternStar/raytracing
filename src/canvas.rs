use crate::color::{Color, ppm_format};

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y * self.width + x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    pub fn to_ppm(&self) -> String {
        let mut s = String::new();
        s.push_str("P3\n");
        s.push_str(&format!("{} {}\n", self.width, self.height));
        s.push_str("255\n");

        let mut lines: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut col: usize = 0;
        for co in &self.pixels {
            let pixel = ppm_format(*co);
            col += 1;
            if current.is_empty() {
                current = pixel;
            } else if col > self.width {
                lines.push(current);
                current = pixel;
                col = 1;
            } else if current.len() + 1 + pixel.len() <= 70 {
                current.push(' ');
                current.push_str(&pixel);
            } else {
                let mut rest = String::new();
                let mut overflow = false;
                for (i, value) in pixel.split(' ').enumerate() {
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
        let body = lines.join("\n") + "\n";
        s.push_str(&body);
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blank() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.height, 20);
        assert_eq!(c.width, 10);
        let zero = Color::new(0.0, 0.0, 0.0);
        for co in c.pixels {
            assert_eq!(co, zero)
        }
    }

    #[test]
    fn test_write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn test_ppm_header() {
        let c = Canvas::new(5, 3);
        let s = c.to_ppm();
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255")
    }

    #[test]
    fn test_ppm_body() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let s = c.to_ppm();
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn test_long_line() {
        let mut c = Canvas::new(10, 2);
        let c1 = Color::new(1.0, 0.8, 0.6);
        for co in &mut c.pixels {
            *co = c1;
        }
        let s = c.to_ppm();
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines[3], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
        assert_eq!(lines[4], "153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert_eq!(lines[5], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
        assert_eq!(lines[6], "153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert!(s.ends_with('\n'));
    }
}
