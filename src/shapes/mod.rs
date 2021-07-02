pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod plane;
pub mod sphere;

use std::fmt::Debug;
use uuid::Uuid;

use crate::{
    intersection::Intersection, materials::Material, ray::Ray, transformations::Transform,
    tuple::Tuple,
};

pub trait ShapeBuilder {
    fn with_material(self, material: Material) -> Self;
    fn with_transform(self, transform: Transform) -> Self;
}

pub trait Shape: 'static + Debug {
    fn id(&self) -> Uuid;

    fn shape_eq(&self, other: &dyn Shape) -> bool {
        self.id() == other.id()
    }

    fn transform(&self) -> &Transform;
    fn set_transform(&mut self, transform: Transform);

    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn set_material(&mut self, material: Material);

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, point: Tuple) -> Tuple;

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.transform().inverse());
        self.local_intersect(local_ray)
    }

    fn normal_at(&self, x: f32, y: f32, z: f32) -> Tuple {
        let world_point = Tuple::point(x, y, z);
        let local_point = self.transform().inverse() * world_point;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = self.transform().inverse().transpose() * local_normal;
        world_normal.normalize()
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
