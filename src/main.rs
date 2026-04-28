use raytracing::vector::{Vector, normalize};

fn tick(env: (Vector, Vector), projectile: (Vector, Vector)) -> (Vector, Vector) {
    let pos = projectile.0 + projectile.1;
    let v = projectile.1 + env.0 + env.1;
    (pos, v)
}

fn main() {
    let env = (Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));
    let mut projectile = (Vector::new(0.0, 1.0, 0.0), normalize(Vector::new(1.0, 1.0, 0.0)));
    while projectile.0.0.y > 0.0 {
        projectile = tick(env, projectile);
        dbg!(projectile);
    }
}
