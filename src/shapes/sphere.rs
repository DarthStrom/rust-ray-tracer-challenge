use uuid::Uuid;

use crate::{
    intersection::Intersection,
    materials::Material,
    ray::Ray,
    shapes::{Shape, ShapeBuilder},
    transformations::{Transform, IDENTITY},
    tuple::Tuple,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    id: Uuid,
    parent: Option<Uuid>,
    transform: Transform,
    material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn glass() -> Self {
        Self::default().with_material(Material::default().transparency(1.0).refractive_index(1.5))
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
            transform: IDENTITY,
            material: Material::default(),
        }
    }
}

impl ShapeBuilder for Sphere {
    fn with_transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }

    fn with_material(self, material: Material) -> Self {
        Self { material, ..self }
    }
}

impl Shape for Sphere {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn parent(&self) -> Option<Uuid> {
        self.parent
    }

    fn set_parent(&mut self, parent: Uuid) {
        self.parent = Some(parent);
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let discriminant = discriminant(ray);

        if discriminant < 0.0 {
            vec![]
        } else {
            let a = a(ray);
            let b = b(ray);
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            let i1 = Intersection::new(t1, self);
            let i2 = Intersection::new(t2, self);
            vec![i1, i2]
        }
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        point - Tuple::point(0.0, 0.0, 0.0)
    }
}

fn a(ray: Ray) -> f32 {
    ray.direction.dot(ray.direction)
}

fn b(ray: Ray) -> f32 {
    let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
    2.0 * ray.direction.dot(sphere_to_ray)
}

fn c(ray: Ray) -> f32 {
    let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
    sphere_to_ray.dot(sphere_to_ray) - 1.0
}

fn discriminant(ray: Ray) -> f32 {
    b(ray).powi(2) - 4.0 * a(ray) * c(ray)
}

#[cfg(test)]
mod tests {
    use crate::{
        float_eq,
        test::*,
        transformations::{self, IDENTITY},
    };
    use std::f32::consts::{FRAC_1_SQRT_2, PI};

    use super::*;

    #[test]
    fn default_material() {
        let s = Sphere::default();

        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn changing_material() {
        let m = Material {
            ambient: 1.0,
            ..Material::default()
        };
        let s = Sphere::default().with_material(m.clone());

        assert_eq!(s.material, m);
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(float_eq(xs[0].t, 4.0));
        assert!(float_eq(xs[1].t, 6.0));
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(float_eq(xs[0].t, 5.0));
        assert!(float_eq(xs[1].t, 5.0));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originating_inside_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(float_eq(xs[0].t, -1.0));
        assert!(float_eq(xs[1].t, 1.0));
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(float_eq(xs[0].t, -6.0));
        assert!(float_eq(xs[1].t, -4.0));
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::default();

        assert_eq!(s.transform, IDENTITY);
    }

    #[test]
    fn changin_a_spheres_transformation() {
        let t = Transform::translation(2.0, 3.0, 4.0);
        let s = Sphere::default().with_transform(t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let s = Sphere::default().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(float_eq(xs[0].t, 3.0));
        assert!(float_eq(xs[1].t, 7.0));
    }

    #[test]
    fn normal_at_a_point_on_x_axis() {
        let s = Sphere::default();

        let n = s.normal_at(1.0, 0.0, 0.0);

        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_at_a_point_on_y_axis() {
        let s = Sphere::default();

        let n = s.normal_at(0.0, 1.0, 0.0);

        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_at_a_point_on_z_axis() {
        let s = Sphere::default();

        let n = s.normal_at(0.0, 0.0, 1.0);

        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_at_a_nonaxial_point() {
        let s = Sphere::default();

        let n = s.normal_at(sqrt_n_over_n(3), sqrt_n_over_n(3), sqrt_n_over_n(3));

        assert_eq!(
            n,
            Tuple::vector(sqrt_n_over_n(3), sqrt_n_over_n(3), sqrt_n_over_n(3))
        );
    }

    #[test]
    fn normal_is_normalized() {
        let s = Sphere::default();

        let n = s.normal_at(sqrt_n_over_n(3), sqrt_n_over_n(3), sqrt_n_over_n(3));

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn normal_on_a_translated_sphere() {
        let s = Sphere::default().with_transform(Transform::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2);

        assert_eq!(n, Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_on_a_transformed_sphere() {
        let m = Transform::scaling(1.0, 0.5, 1.0) * Transform::rotation_z(PI / 5.0);
        let s = Sphere::default().with_transform(m);

        let n = s.normal_at(0.0, sqrt_n_over_n(2), -sqrt_n_over_n(2));

        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        let s = Sphere::glass();

        assert_eq!(s.transform, transformations::IDENTITY);
        assert!(float_eq(s.material.transparency, 1.0));
        assert!(float_eq(s.material.refractive_index, 1.5));
    }
}
