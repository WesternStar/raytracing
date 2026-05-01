use raytracing::canvas::Canvas;
use raytracing::color::Color;
use raytracing::vector::Vector;
use std::fs;

fn tick(env: (Vector, Vector), projectile: (Vector, Vector)) -> (Vector, Vector) {
    let pos = projectile.0 + projectile.1;
    let v = projectile.1 + env.0 + env.1;
    (pos, v)
}

fn main() {
    let env = (Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));
    let mut projectile = (Vector::new(0.0, 1.0, 0.0), Vector::new(1.0, 1.0, 0.0).normalize());

    let width = 900usize;
    let height = 550usize;
    let x_scale = width as f64 / 10.0;
    let y_scale = height as f64 / 4.0;

    let mut canvas = Canvas::new(width, height);
    let red = Color::new(1.0, 0.0, 0.0);

    let to_canvas = |x: f64, y: f64| -> Option<(usize, usize)> {
        let cx = (x * x_scale).round() as usize;
        let scaled_y = (y * y_scale).round() as usize;
        if cx < width && scaled_y < height {
            Some((cx, height - 1 - scaled_y))
        } else {
            None
        }
    };

    let mut prev = to_canvas(projectile.0.0.x, projectile.0.0.y);

    while projectile.0.0.y > 0.0 {
        projectile = tick(env, projectile);
        let curr = to_canvas(projectile.0.0.x, projectile.0.0.y);
        if let (Some((x0, y0)), Some((x1, y1))) = (prev, curr) {
            let steps = ((x1 as i64 - x0 as i64).abs()).max((y1 as i64 - y0 as i64).abs()).max(1);
            for i in 0..=steps {
                let t = i as f64 / steps as f64;
                let x = (x0 as f64 + t * (x1 as f64 - x0 as f64)).round() as usize;
                let y = (y0 as f64 + t * (y1 as f64 - y0 as f64)).round() as usize;
                if x < width && y < height {
                    canvas.write_pixel(x, y, red);
                }
            }
        }
        prev = curr;
    }

    let ppm = canvas.to_ppm();
    fs::write("projectile.ppm", ppm).expect("Failed to write file");
    println!("Written to projectile.ppm");
}
