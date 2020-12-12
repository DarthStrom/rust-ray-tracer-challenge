use tuple::Tuple;

mod canvas;
mod color;
mod tuple;

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalized() * 2.0,
    };
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut tick_count = 0;
    while p.position.y > 0.0 {
        p = tick(&e, &p);
        tick_count += 1;
        println!("Position: {:?}", p.position)
    }
    println!("Ticks until ground: {:?}", tick_count);
}

struct Projectile {
    position: Tuple<f32>,
    velocity: Tuple<f32>,
}

struct Environment {
    gravity: Tuple<f32>,
    wind: Tuple<f32>,
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}
