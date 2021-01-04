use std::fmt::Debug;

use plane::Plane;
use sphere::Sphere;

use crate::{
    ray::{intersections::Intersections, Ray},
    tuple::Tuple,
};

pub mod plane;
pub mod sphere;

pub trait Shape: Debug {
    fn normal_at(&self, x: f64, y: f64, z: f64) -> Result<Tuple, String>;
    fn intersect(&self, ray: Ray) -> Intersections;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Shapes {
    Plane(Plane),
    Sphere(Sphere),
}

impl Shape for Shapes {
    fn intersect(&self, ray: Ray) -> Intersections {
        match self {
            Shapes::Plane(plane) => plane.intersect(ray),
            Shapes::Sphere(sphere) => sphere.intersect(ray),
        }
    }

    fn normal_at(&self, x: f64, y: f64, z: f64) -> Result<Tuple, String> {
        match self {
            Shapes::Plane(plane) => plane.normal_at(x, y, z),
            Shapes::Sphere(sphere) => sphere.normal_at(x, y, z),
        }
    }
}
