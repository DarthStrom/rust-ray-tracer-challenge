use std::fmt::Debug;

use plane::Plane;
use sphere::Sphere;

use crate::{material::Material, matrix::transform::Transform, ray::{intersections::Intersections, Ray}, tuple::Tuple};

pub mod plane;
pub mod sphere;

pub trait Shape: Debug {
    fn get_material(&self) -> Material;
    fn get_transform(&self) -> Transform;
    fn intersect(&self, ray: Ray) -> Intersections;
    fn normal_at(&self, x: f64, y: f64, z: f64) -> Result<Tuple, String>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Plane(Plane),
    Sphere(Sphere),
}

impl Shape for Object {
    fn get_material(&self) -> Material {
        match self {
            Object::Plane(plane) => plane.get_material(),
            Object::Sphere(sphere) => sphere.get_material(),
        }
    }
    fn get_transform(&self) -> Transform {
        match self {
            Object::Plane(plane) => plane.get_transform(),
            Object::Sphere(sphere) => sphere.get_transform(),
        }
    }

    fn intersect(&self, ray: Ray) -> Intersections {
        match self {
            Object::Plane(plane) => plane.intersect(ray),
            Object::Sphere(sphere) => sphere.intersect(ray),
        }
    }

    fn normal_at(&self, x: f64, y: f64, z: f64) -> Result<Tuple, String> {
        match self {
            Object::Plane(plane) => plane.normal_at(x, y, z),
            Object::Sphere(sphere) => sphere.normal_at(x, y, z),
        }
    }
}
