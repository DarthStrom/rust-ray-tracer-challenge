#![allow(dead_code)]

mod canvas;
mod color;
mod tuple;

use std::fs;

use canvas::*;
use tuple::*;

#[derive(Clone, Copy)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Clone, Copy)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);
    while p.position.y() > 0.0 {
        p = tick(e, p);

        canvas.write_pixel(
            p.position.x().round() as usize,
            canvas.height - p.position.y().round() as usize,
            color::WHITE,
        );
    }

    fs::write("canvas.ppm", canvas.to_ppm()).unwrap();
}
