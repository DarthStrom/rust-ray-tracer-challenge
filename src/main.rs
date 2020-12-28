use std::fs;

use camera::Camera;
use canvas::Canvas;
use color::Color;
use float_cmp::F64Margin;
use light::PointLight;
use material::Material;
use matrix::transform::Transform;
use ray::Ray;
use sphere::Sphere;
use std::f64::consts::PI;
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
mod ray;
mod sphere;
mod tuple;
mod world;

#[cfg(test)]
mod test;

fn main() {
    let mut floor = Sphere::default();
    floor.set_transform(Transform::scaling(10.0, 0.01, 10.0));
    floor.material = Material::default()
        .color(Color::new(1.0, 0.9, 0.9))
        .specular(0.0);

    let mut left_wall = Sphere::default();
    left_wall.set_transform(
        Transform::identity()
            .scale(10.0, 0.01, 10.0)
            .rotate_x(PI / 2.0)
            .rotate_y(-PI / 4.0)
            .translate(0.0, 0.0, 5.0),
    );
    left_wall.material = floor.material;

    let mut right_wall = Sphere::default();
    right_wall.set_transform(
        Transform::identity()
            .scale(10.0, 0.01, 10.0)
            .rotate_x(PI / 2.0)
            .rotate_y(PI / 4.0)
            .translate(0.0, 0.0, 5.0),
    );
    right_wall.material = floor.material;

    let mut middle = Sphere::default();
    middle.set_transform(Transform::translation(-0.5, 1.0, 0.5));
    middle.material = Material::default()
        .color(Color::new(0.1, 1.0, 0.5))
        .diffuse(0.7)
        .specular(0.3);

    let mut right = Sphere::default();
    right.set_transform(Transform::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5));
    right.material = Material::default()
        .color(Color::new(0.5, 1.0, 0.1))
        .diffuse(0.7)
        .specular(0.3);

    let mut left = Sphere::default();
    left.set_transform(Transform::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75));
    left.material = Material::default()
        .color(Color::new(1.0, 0.8, 0.1))
        .diffuse(0.7)
        .specular(0.3);

    let mut world = World::new();
    world.objects = vec![floor, left_wall, right_wall, middle, right, left];
    world.light_sources = vec![PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    )];

    let mut camera = Camera::new(100, 50, PI / 3.0);
    camera.transform = Transform::view_transform(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    );

    let c = camera.render(&world);

    fs::write("canvas.ppm", c.to_ppm()).unwrap();
}
