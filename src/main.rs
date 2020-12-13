use std::fs;

use canvas::Canvas;
use color::Color;
use tuple::Tuple;

mod canvas;
mod color;
mod matrix;
mod tuple;

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalized() * 11.25,
    };
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut c = Canvas::new(900, 550);
    while p.position.y > 0.0 {
        p = tick(&e, &p);
        println!("writing to {},{}", p.position.x, p.position.y);
        c.write_pixel(
            p.position.x as usize,
            c.height - p.position.y as usize,
            Color::new(0.5, 0.5, 0.5),
        );
    }
    fs::write("canvas.ppm", c.to_ppm()).unwrap();
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
