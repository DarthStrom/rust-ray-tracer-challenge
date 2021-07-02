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

    fn parent(&self) -> Option<Uuid>;

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

#[cfg(test)]
#[derive(Debug, Default)]
pub struct TestShape {
    pub parent: Option<Uuid>,
}

#[cfg(test)]
impl TestShape {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
impl Shape for TestShape {
    fn id(&self) -> Uuid {
        todo!()
    }

    fn transform(&self) -> &Transform {
        todo!()
    }

    fn set_transform(&mut self, transform: Transform) {
        todo!()
    }

    fn material(&self) -> &Material {
        todo!()
    }

    fn material_mut(&mut self) -> &mut Material {
        todo!()
    }

    fn set_material(&mut self, material: Material) {
        todo!()
    }

    fn parent(&self) -> Option<Uuid> {
        self.parent
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        todo!()
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_shape_has_a_parent_attribute() {
        let s = TestShape::new();

        assert_eq!(s.parent(), None);
    }
}
