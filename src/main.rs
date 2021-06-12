#![allow(dead_code)]

mod camera;
mod canvas;
mod color;
mod intersection;
mod lights;
mod materials;
mod ray;
mod sphere;
mod transformations;
mod tuple;
mod world;

#[cfg(test)]
mod test;

use std::{f32::consts::PI, fs};

use camera::Camera;
use color::Color;
use lights::PointLight;
use materials::Material;
use sphere::Sphere;
use transformations::Transform;
use tuple::*;
use world::World;

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
    let floor_material = Material::default()
        .color(Color::new(1.0, 0.9, 0.9))
        .specular(0.0);

    let floor = Sphere::default()
        .transform(Transform::scaling(10.0, 0.01, 10.0))
        .material(floor_material);

    let left_wall = Sphere::default()
        .transform(
            Transform::translation(0.0, 0.0, 5.0)
                * Transform::rotation_y(-PI / 4.0)
                * Transform::rotation_x(PI / 2.0)
                * Transform::scaling(10.0, 0.01, 10.0),
        )
        .material(floor_material);

    let right_wall = Sphere::default()
        .transform(
            Transform::translation(0.0, 0.0, 5.0)
                * Transform::rotation_y(PI / 4.0)
                * Transform::rotation_x(PI / 2.0)
                * Transform::scaling(10.0, 0.01, 10.0),
        )
        .material(floor_material);

    let middle = Sphere::default()
        .transform(Transform::translation(-0.5, 1.0, 0.5))
        .material(
            Material::default()
                .color(Color::new(0.1, 1.0, 0.5))
                .diffuse(0.7)
                .specular(0.3),
        );

    let right = Sphere::default()
        .transform(Transform::translation(1.5, 0.5, -0.5) * Transform::scaling(0.5, 0.5, 0.5))
        .material(
            Material::default()
                .color(Color::new(0.5, 1.0, 0.1))
                .diffuse(0.7)
                .specular(0.3),
        );

    let left = Sphere::default()
        .transform(Transform::translation(-1.5, 0.33, -0.75) * Transform::scaling(0.33, 0.33, 0.33))
        .material(
            Material::default()
                .color(Color::new(1.0, 0.8, 0.1))
                .diffuse(0.7)
                .specular(0.3),
        );

    let world = World::new()
        .light_source(PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            color::WHITE,
        ))
        .objects(&[floor, left_wall, right_wall, left, middle, right]);

    let camera = Camera::new(100, 50, PI / 3.0).transform(Transform::view_transform(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);

    fs::write("canvas.ppm", canvas.to_ppm()).unwrap();
}
