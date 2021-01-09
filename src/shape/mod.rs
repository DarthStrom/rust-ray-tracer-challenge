use std::fmt::Debug;

use plane::Plane;
use sphere::Sphere;

use crate::{
    matrix::transform::Transform,
    ray::{intersections::Intersections, Ray},
    tuple::Tuple,
};

pub mod plane;
pub mod sphere;

pub trait Shape: Debug {
    fn normal_at(&self, x: f64, y: f64, z: f64) -> Result<Tuple, String>;
    fn intersect(&self, ray: Ray) -> Intersections;
    fn transform(&self) -> Transform;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Plane(Plane),
    Sphere(Sphere),
}

impl Shape for Object {
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

    fn transform(&self) -> Transform {
        match self {
            Object::Plane(plane) => plane.transform.clone(),
            Object::Sphere(sphere) => sphere.transform.clone(),
        }
    }
}
