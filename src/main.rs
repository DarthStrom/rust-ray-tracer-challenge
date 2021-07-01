#![allow(dead_code)]

mod camera;
mod canvas;
mod color;
mod intersection;
mod lights;
mod materials;
mod patterns;
mod ray;
mod shapes;
mod transformations;
mod tuple;
mod world;

#[cfg(test)]
mod test;

use std::cmp::Ordering;
use std::{f32::consts::PI, fs};

use camera::Camera;
use color::Color;
use lights::PointLight;
use materials::Material;
use shapes::cone::Cone;
use shapes::cylinder::Cylinder;
use shapes::plane::Plane;
use shapes::sphere::Sphere;
use shapes::ShapeBuilder;
use transformations::Transform;
use tuple::*;
use world::World;

pub const EPSILON: f32 = 0.0001;

pub fn float_eq(x: f32, y: f32) -> bool {
    (y - x).abs() < EPSILON
}

pub fn float_cmp(x: f32, y: f32) -> Ordering {
    if float_eq(x, y) {
        Ordering::Equal
    } else if x < y {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

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
        .specular(0.0)
        .reflective(0.8);

    let floor = Plane::default().with_material(floor_material);

    let middle = Sphere::default()
        .with_transform(Transform::translation(-0.5, 1.0, 0.5))
        .with_material(
            Material::default()
                .color(Color::new(0.1, 1.0, 0.5))
                .diffuse(0.2)
                .ambient(0.1)
                .specular(0.3)
                .reflective(0.9)
                .transparency(0.9),
        );

    let right = Cone::default()
        .with_transform(Transform::translation(1.5, 0.0, -0.5) * Transform::scaling(0.5, 0.5, 0.5))
        .with_material(
            Material::default()
                .color(Color::new(0.5, 1.0, 0.1))
                .diffuse(0.7)
                .specular(0.3)
                .reflective(0.2),
        )
        .with_caps(0.0, 1.5);

    let left = Cylinder::default()
        .with_transform(
            Transform::translation(-1.5, 0.33, -0.75) * Transform::scaling(0.33, 0.33, 0.33),
        )
        .with_material(
            Material::default()
                .color(Color::new(1.0, 0.8, 0.1))
                .diffuse(0.7)
                .specular(0.3),
        )
        .with_caps(-1.0, 3.0);

    let world = World::new(PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        color::WHITE,
    ))
    .object(Box::new(floor))
    .object(Box::new(left))
    .object(Box::new(middle))
    .object(Box::new(right));

    let camera = Camera::new(1000, 500, PI / 3.0).transform(Transform::view_transform(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);

    fs::write("canvas.ppm", canvas.to_ppm()).unwrap();
}
