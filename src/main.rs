#![allow(dead_code)]

use camera::Camera;
use color::Color;
use float_cmp::F64Margin;

use light::PointLight;
use material::Material;
use matrix::transform::Transform;
use shape::{plane::Plane, sphere::Sphere, Object};
use std::{f64::consts::PI, fs};
use tuple::Tuple;
use world::World;

pub const MARGIN: F64Margin = F64Margin {
    ulps: 2,
    epsilon: 0.00001,
};

#[cfg(test)]
macro_rules! f_assert_eq {
    ($a:expr, $b:expr) => {
        assert!($a.approx_eq($b, $crate::MARGIN));
    };
}

mod camera;
mod canvas;
mod color;
mod light;
mod material;
mod matrix;
mod pattern;
mod ray;
mod shape;
mod tuple;
mod world;

#[cfg(test)]
mod test;

// TODO: make the API more consistent based on this usage

#[allow(unused_variables)]
fn main() {
    let floor_material = Material::default()
        .color(Color::new(1.0, 0.9, 0.9))
        .specular(0.0);
    let floor = Object::Plane(Plane::default().material(floor_material));

    let middle = Object::Sphere(
        Sphere::default()
            .transform(Transform::translation(-0.5, 1.0, 0.5))
            .material(
                Material::default()
                    .color(Color::new(0.1, 1.0, 0.5))
                    .diffuse(0.7)
                    .specular(0.3),
            ),
    );

    let right = Object::Sphere(
        Sphere::default()
            .transform(Transform::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5))
            .material(
                Material::default()
                    .color(Color::new(0.5, 1.0, 0.1))
                    .diffuse(0.7)
                    .specular(0.3),
            ),
    );

    let left = Object::Sphere(
        Sphere::default()
            .transform(Transform::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75))
            .material(
                Material::default()
                    .color(Color::new(1.0, 0.8, 0.1))
                    .diffuse(0.7)
                    .specular(0.3),
            ),
    );

    let world = World::new()
        .objects(&[floor, middle, right, left])
        .light_sources(&[PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        )]);

    let camera = Camera::new(100, 50, PI / 3.0).transform(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    );

    let c = camera.render(&world);

    fs::write("canvas.ppm", c.to_ppm()).unwrap();
}
