use ray_tracer_challenge::{Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector
}

struct Environment {
    gravity: Vector,
    wind_force: Vector,
}

fn tick(env: &Environment, projectile: &mut Projectile) {
    projectile.position = projectile.position + projectile.velocity;
    projectile.velocity = projectile.velocity + env.gravity + env.wind_force;
}

fn main() {
    let mut p = Projectile{
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0),
    };
    let e = Environment{
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind_force: Vector::new(-0.01, 0.0, 0.0),
    };

    while p.position.0.1 > 0.0 {
        tick(&e, &mut p);
        println!("Projectile current position: {}", p.position);
    }

    println!("Projectile final position: {}", p.position);
}