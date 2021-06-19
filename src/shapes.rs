use std::fmt::Debug;

use crate::{
    intersection::Intersections, materials::Material, ray::Ray, transformations::Transform,
    tuple::Tuple,
};

pub trait ShapeBuilder {
    fn with_material(self, material: Material) -> Self;
    fn with_transform(self, transform: Transform) -> Self;
}

pub trait Shape: Debug {
    fn material(&self) -> Material;
    fn transform(&self) -> Transform;
    fn normal_at(&self, x: f32, y: f32, z: f32) -> Tuple;
    fn intersect(&self, ray: Ray) -> Intersections;
}

// The book has you write these tests, but I'd rather keep them in the impl files
//
// #[cfg(test)]
// mod tests {
//     use crate::tuple::Tuple;

//     use super::*;

//     #[derive(Copy, Clone, Default)]
//     struct TestShape {
//         material: Material,
//         saved_ray: Ray,
//         transform: Transform,
//     }

//     impl Shape for TestShape {
//         fn intersect(&self, ray: Ray) -> Intersections {
//             Intersections::new(vec![])
//         }

//         fn material(self, material: Material) -> Self {
//             Self { material, ..self }
//         }

//         fn transform(self, transform: Transform) -> Self {
//             Self { transform, ..self }
//         }
//     }

//     #[test]
//     fn the_default_transformation() {
//         let s = TestShape::default();

//         assert_eq!(s.transform, transformations::IDENTITY);
//     }

//     #[test]
//     fn assigning_a_transformation() {
//         let mut s = TestShape::default();

//         s = s.transform(Transform::translation(2.0, 3.0, 4.0));

//         assert_eq!(s.transform, Transform::translation(2.0, 3.0, 4.0))
//     }

//     #[test]
//     fn the_default_material() {
//         let s = TestShape::default();

//         assert_eq!(s.material, Material::default());
//     }

//     #[test]
//     fn assigning_a_material() {
//         let mut s = TestShape::default();
//         let m = Material::default().ambient(1.0);

//         s = s.material(m);

//         assert_eq!(s.material, m)
//     }

//     #[test]
//     fn intersecting_a_scaled_shape_with_a_ray() {
//         let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
//         let mut s = TestShape::default();

//         s = s.transform(Transform::scaling(2.0, 2.0, 2.0));
//         let xs = s.intersect(r);

//         assert_eq!(
//             s.saved_ray,
//             Ray::new(Tuple::point(0.0, 0.0, -2.5), Tuple::vector(0.0, 0.0, 0.5))
//         );
//     }
// }
