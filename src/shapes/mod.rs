pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod sphere;

use std::{any::Any, fmt::Debug};

use crate::{
    intersection::Intersections, materials::Material, ray::Ray, transformations::Transform,
    tuple::Tuple,
};

pub trait ShapeBuilder {
    fn with_material(self, material: Material) -> Self;
    fn with_transform(self, transform: Transform) -> Self;
}

pub trait Shape: Any + Debug {
    fn box_clone(&self) -> BoxShape;
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn material(&self) -> &Material;
    fn transform(&self) -> &Transform;
    fn normal_at(&self, x: f32, y: f32, z: f32) -> Tuple;
    fn intersect(&self, ray: Ray) -> Intersections;
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.box_eq(other.as_any())
    }
}

pub type BoxShape = Box<dyn Shape>;

impl Clone for BoxShape {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
